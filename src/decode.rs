use std::io::{Cursor, Error, Read};
use byteorder::{BigEndian, ReadBytesExt};
use serde::de::Deserializer;
use serde_bytes;

use super::encode::{EncodeError, Header, HeaderLayout, Strategy, StrategyDataTypes};
use super::encoding::{IntegerEncoding, RunLength};
use super::codec::{DeltaRunlength, IntegerDeltaRecursive, IntegerRunLength};
use super::binary_decoder;


#[derive(Debug)]
struct Decoder<'a> {
    reader: Cursor<&'a [u8]>,
}

impl<'a> Decoder<'a> {
    fn new(reader: &'a [u8]) -> Self {
        let reader = Cursor::new(reader);
        Decoder { reader }
    }

    fn field(&mut self) -> Result<Vec<u8>, Error> {
        // Skip header bytes
        self.reader.set_position(12);
        let mut buffer: Vec<u8> = Vec::new();
        try!(self.reader.read_to_end(&mut buffer));
        Ok(buffer)
    }
}

impl<'a> Header for Decoder<'a> {
    fn header(&mut self) -> Result<HeaderLayout, EncodeError> {
        self.reader.set_position(0);

        // Will return an error if the position is not at the Start
        // and the number of the bytes is less than 12.
        let header_len = self.reader.get_ref().len();
        if header_len < 12 {
            let err = format!("bytes length should be more than {}", header_len);
            return Err(EncodeError::Header(err));
        } else {
            let codec = self.reader.read_i32::<BigEndian>().unwrap();
            let length = self.reader.read_i32::<BigEndian>().unwrap();
            let parameter = self.reader.read_i32::<BigEndian>().unwrap();

            Ok(HeaderLayout {
                codec,
                length,
                parameter,
            })
        }
    }
}

impl<'a> Strategy for Decoder<'a> {
    fn apply(&mut self) -> Result<StrategyDataTypes, EncodeError> {
        let header = try!(self.header());
        let field = self.field().unwrap();

        match header.codec {
            1 => {
                let decoded: Vec<f32> = try!(binary_decoder::Interpret::from(&field[..]));
                Ok(StrategyDataTypes::VecFloat32(decoded))
            }
            2 => Ok(StrategyDataTypes::VecInt8(
                try!(binary_decoder::Interpret::from(&field[..])),
            )),
            3 => Ok(StrategyDataTypes::VecInt16(
                try!(binary_decoder::Interpret::from(&field[..])),
            )),
            4 => Ok(StrategyDataTypes::VecInt32(
                try!(binary_decoder::Interpret::from(&field[..])),
            )),
            5 => {
                let result: Vec<String> = try!(binary_decoder::Interpret::from(&field[..]));
                Ok(StrategyDataTypes::VecString(result))
            }
            6 => {
                let asi32: Vec<i32> = try!(binary_decoder::Interpret::from(&field[..]));
                let run = try!(RunLength::decode(&asi32));
                let result: Vec<char> = try!(binary_decoder::Interpret::from(&run[..]));
                Ok(StrategyDataTypes::VecChar(result))
            }
            7 => {
                let bytes: Vec<i32> = try!(binary_decoder::Interpret::from(&field[..]));
                let result = try!(RunLength::decode(&bytes));
                Ok(StrategyDataTypes::VecInt32(result))
            }
            8 => {
                let result = try!(DeltaRunlength::decode(&field));
                Ok(StrategyDataTypes::VecInt32(result))
            }
            9 => {
                let result = try!(IntegerRunLength::decode(&field, header.parameter));
                Ok(StrategyDataTypes::VecFloat32(result))
            }
            10 => {
                let result = IntegerDeltaRecursive::decode(&field[..], header.parameter);
                Ok(StrategyDataTypes::VecFloat32(result))
            }
            11 => {
                let values: Vec<i16> = try!(binary_decoder::Interpret::from(&field[..]));
                let result = IntegerEncoding::decode(&values, header.parameter);
                Ok(StrategyDataTypes::VecFloat32(result))
            }
            12 => unimplemented!(),
            13 => unimplemented!(),
            14 => unimplemented!(),
            15 => unimplemented!(),
            _ => Err(EncodeError::Codec),
        }
    }
}

/// Deserialize the encoded sequence of values
///
/// This function is generic over T which can be any type that implements
/// From<StrategyDataTypes>
pub fn as_decoder<'de, T, D>(deserialize: D) -> Result<T, D::Error>
where
    T: From<StrategyDataTypes>,
    D: Deserializer<'de>,
{
    let re: Vec<u8> = serde_bytes::deserialize(deserialize)?;
    let decoded = Decoder::new(&re).apply().unwrap();
    Ok(From::from(decoded))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_create_new_decoder() {
        let data = [1, 2, 3];
        let decoder = Decoder::new(&data);
        assert_eq!([1, 2, 3], decoder.reader.into_inner());
    }

    #[test]
    fn it_parse_header_success() {
        let data = vec![0, 0, 0, 4, 0, 0, 0, 52, 0, 0, 0, 0];
        let mut decoder = Decoder::new(&data);
        let header = decoder.header().unwrap();

        assert_eq!(4, header.codec);
        assert_eq!(52, header.length);
        assert_eq!(0, header.parameter);
    }

    #[test]
    fn it_parse_header_fail() {
        let data = vec![0, 0, 0, 4, 0, 0, 0, 52, 0, 0, 0];
        let mut decoder = Decoder::new(&data);
        let header = decoder.header();

        if let EncodeError::Header(err) = header.unwrap_err() {
            assert_eq!(err, "bytes length should be more than 11");
        } else {
            panic!();
        }
    }

    #[test]
    fn test_parse_field() {
        let data = [
            0, 0, 0, 2, 0, 0, 0, 26, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1
        ];
        let expected = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let mut decoder = Decoder::new(&data);
        decoder.header();

        let actual = decoder.field().unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_apply_strategy_for_type_1() {
        let data = [
            0, 0, 0, 1, 0, 0, 0, 2, 0, 0, 0, 0, 63, 153, 153, 154, 64, 57, 153, 154
        ];
        let expected = vec![1.2, 2.9];
        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecFloat32(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }

    #[test]
    fn test_apply_strategy_for_type_2() {
        let data = [
            0, 0, 0, 2, 0, 0, 0, 10, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
        ];
        let expected = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecInt8(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }

    #[test]
    fn test_apply_strategy_for_type_3() {
        let data = [0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 10, 0, 20, 0, 22];
        let expected = vec![10, 20, 22];
        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecInt16(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }

    #[test]
    fn test_apply_strategy_for_type_4() {
        let data = [
            0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 5, 0, 0, 0, 40, 0, 0, 0, 27
        ];
        let expected = vec![19, 5, 40, 27];
        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecInt32(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }

    #[test]
    fn test_apply_strategy_for_type_5() {
        let data = [
            0, 0, 0, 5, 0, 0, 0, 8, 0, 0, 0, 4, 65, 0, 0, 0, 66, 0, 0, 0, 67, 0, 0, 0, 68, 0, 0, 0,
            69, 0, 0, 0, 70, 0, 0, 0, 71, 0, 0, 0, 72, 0, 0, 0,
        ];
        let expected = vec!["A", "B", "C", "D", "E", "F", "G", "H"];
        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecString(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }

    #[test]
    fn test_apply_strategy_for_type_6() {
        let data = [0, 0, 0, 6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 65, 0, 0, 0, 3];
        let expected = vec!['A', 'A', 'A'];
        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecChar(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }

    #[test]
    fn test_apply_strategy_for_type_8() {
        let data = [
            0, 0, 0, 8, 0, 0, 0, 124, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7,
            255, 255, 255, 249, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 7, 255, 255, 255, 249, 0, 0, 0, 1,
            0, 0, 0, 1, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 5, 255, 255, 255,
            245, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 95,
        ];

        let expected = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 5, 6, 7, 8, 9, 10,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
            -1, -1, -1, -1, -1, -1, -1, -1,
        ];

        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecInt32(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }

    #[test]
    fn test_apply_strategy_for_type_9() {
        let data = [
            0, 0, 0, 9, 0, 0, 0, 9, 0, 0, 0, 100, 0, 0, 0, 150, 0, 0, 0, 1, 0, 0, 1, 24, 0, 0, 0,
            1, 0, 0, 0, 150, 0, 0, 0, 3, 0, 0, 0, 250, 0, 0, 0, 3,
        ];

        let expected = vec![1.5, 2.8, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5];

        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecFloat32(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }
}
