use std::io::Error;
use std::iter;
use std::io::Cursor;
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt, ByteOrder};

trait Decode {
    fn decode(&mut self);
}

struct Decoder<'a> {
    reader: Cursor<&'a [u8]>,
}

struct Header {
    codec: i32,
    length: i32,
    parameter: i32,
}

impl<'a> Decoder<'a> {
    fn new(reader: &'a [u8]) -> Self {
        let reader = Cursor::new(reader);
        Decoder { reader }
    }

    fn header(&mut self) -> Result<Header, Error> {
        let codec = self.reader.read_i32::<BigEndian>().unwrap();
        let length = self.reader.read_i32::<BigEndian>().unwrap();
        let parameter = self.reader.read_i32::<BigEndian>().unwrap();

        Ok(Header { codec, length, parameter })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_create_new_decoder() {
        let data = [1,2,3];
        let decoder = Decoder::new(&data);
        assert_eq!([1,2,3], decoder.reader.into_inner());
    }

    #[test]
    fn it_parse_header() {
        let data = vec![0, 0, 0, 4, 0, 0, 0, 52, 0, 0, 0, 0];
        let mut decoder = Decoder::new(&data);

        let Header {codec, length, parameter } = decoder.header().unwrap();

        assert_eq!(4, codec);
        assert_eq!(52, length);
        assert_eq!(0, parameter);
    }
}
