use std::iter;
use std::io::Cursor;
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};

use super::encode::{Strategy, StrategyDataTypes};
use super::codec::{Delta, RunLength, Integer};
use super::binary_decoder;

trait Decode {
    fn decode(&mut self);
}

struct Decoder<'a> {
    reader: Cursor<&'a [u8]>,
}

#[derive(Debug)]
struct Header {
    codec: i32,
    length: i32,
    parameter: i32,
}

impl Header {
    fn read_info(decoder: &mut Decoder) -> Result<Self, &'static str> {
        // Will return an error if the position is not at the Start
        // and the number of the bytes is less than 12.
        if decoder.reader.position() > 0 || decoder.reader.get_ref().len() < 12 {
            Err("The reader dont contain the minimum number of bytes (12) to parse the Header")
        } else {
            let codec = decoder.reader.read_i32::<BigEndian>().unwrap();
            let length = decoder.reader.read_i32::<BigEndian>().unwrap();
            let parameter = decoder.reader.read_i32::<BigEndian>().unwrap();

            Ok(Header {
                codec,
                length,
                parameter,
            })
        }
    }
}

impl<'a> Decoder<'a> {
    fn new(reader: &'a [u8]) -> Self {
        let reader = Cursor::new(reader);
        Decoder { reader }
    }

    fn read_field(&mut self) -> Result<Vec<u8>, &'static str> {
        let mut buffer: Vec<u8> = Vec::new();
        self.reader.read_to_end(&mut buffer);
        Ok(buffer)
    }
}

impl<'a> Strategy for Decoder<'a> {
    fn apply(&mut self) -> Result<StrategyDataTypes, &'static str> {
        let header = Header::read_info(self).unwrap();
        let field = self.read_field().unwrap();

        match header.codec {
            1 => Ok(StrategyDataTypes::VecFloat32(
                binary_decoder::interpret_bytes_as_f32(&field),
            )),
            2 => Ok(StrategyDataTypes::VecInt8(
                binary_decoder::interpret_bytes_as_i8(&field),
            )),
            3 => Ok(StrategyDataTypes::VecInt16(
                binary_decoder::interpret_bytes_as_i16(&field),
            )),
            4 => Ok(StrategyDataTypes::VecInt32(
                binary_decoder::interpret_bytes_as_i32(&field),
            )),
            5 => Ok(StrategyDataTypes::VecChar(
                binary_decoder::interpret_bytes_as_char(
                    &field,
                    header.parameter as usize,
                ),
            )),
            6 => unimplemented!(),
            7 => unimplemented!(),
            8 => {
                let asi32 = binary_decoder::interpret_bytes_as_i32(&field);
                let runlen = RunLength::decode(&asi32);
                let delta = Delta::decode(&runlen);

                Ok(StrategyDataTypes::VecInt32(delta))
            }
            9 => {
                let asi32 = binary_decoder::interpret_bytes_as_i32(&field);
                let runlen = RunLength::decode(&asi32);
                let integer = Integer::decode(&runlen, header.parameter);

                Ok(StrategyDataTypes::VecFloat32(integer))
            },
            10 => unimplemented!(),
            11 => unimplemented!(),
            12 => unimplemented!(),
            13 => unimplemented!(),
            14 => unimplemented!(),
            15 => unimplemented!(),
            _ => Err("nothing here"),
        }
    }
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
        let header = Header::read_info(&mut decoder).unwrap();

        assert_eq!(4, header.codec);
        assert_eq!(52, header.length);
        assert_eq!(0, header.parameter);
    }

    #[test]
    fn it_parse_header_fail() {
        let mut data = vec![0, 0, 0, 4, 0, 0, 0, 52, 0, 0, 0];
        let mut decoder = Decoder::new(&data);
        let header = Header::read_info(&mut decoder);
        assert_eq!(
            header.unwrap_err(),
            "The reader dont contain the minimum number of bytes (12) to parse the Header"
        );
    }

    #[test]
    fn test_parse_field() {
        let data = [
            0, 0, 0, 2, 0, 0, 0, 26, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1
        ];
        let expected = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
        let mut decoder = Decoder::new(&data);
        Header::read_info(&mut decoder).unwrap();

        let actual = decoder.read_field().unwrap();
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
        let expected = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
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
        let data = [0, 0, 0, 9, 0, 0, 0, 9, 0, 0, 0, 100, 0, 0, 0, 150, 0, 0, 0, 1, 0, 0, 1, 24, 0, 0, 0, 1, 0, 0, 0, 150, 0, 0, 0, 3, 0, 0, 0, 250, 0, 0, 0, 3];

        let expected = vec![1.5, 2.8, 1.5, 1.5, 1.5, 2.5, 2.5, 2.5];

        let mut decoder = Decoder::new(&data);
        if let StrategyDataTypes::VecFloat32(actual) = decoder.apply().unwrap() {
            assert_eq!(expected, actual);
        } else {
            panic!();
        };
    }
}
