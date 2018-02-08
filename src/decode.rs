use std::iter;
use std::io::Cursor;
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt};

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
}
