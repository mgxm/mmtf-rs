use std::io::Error;
use std::iter;
use std::io::Cursor;
use std::io::Read;
use byteorder::{BigEndian, ReadBytesExt, ByteOrder};

trait Decode {
    fn decode(&mut self);
}

struct Decoder<'a> {
    data: Cursor<&'a [u8]>,
}

struct Header {
    codec: i32,
    length: i32,
    parameter: i32,
}

impl<'a> Decoder<'a> {
    fn new(data: &'a [u8]) -> Self {
        let data = Cursor::new(data);
        Decoder { data }
    }

    fn header(&mut self) -> Result<Header, Error> {
        let codec = self.data.read_i32::<BigEndian>().unwrap();
        let length = self.data.read_i32::<BigEndian>().unwrap();
        let parameter = self.data.read_i32::<BigEndian>().unwrap();

        Ok(Header { codec, length, parameter })
    }

    fn as_array_of_i32(&mut self, len: usize) -> Result<Vec<i32>, Error> {
        let mut res = vec![0; len];

        self.data.read_i32_into::<BigEndian>(&mut res);
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_create_new_decoder() {
        let data = [1,2,3];
        let decoder = Decoder::new(&data);
        assert_eq!([1,2,3], decoder.data.into_inner());
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

    #[test]
    fn it_decode_as_arrays_of_i32() {
        let data = vec![0, 0, 0, 4, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 5, 0, 0, 0, 40, 0, 0, 0, 27, 0, 0, 0, 61, 0, 0, 0, 48, 0, 0, 0, 83, 0, 0, 0, 69, 0, 0, 0, 102, 0, 0, 0, 91, 0, 0, 0, 122, 0, 0, 0, 110, 0, 0, 0, 142, 0, 0, 0, 130, 0, 0, 0, 180, 0, 0, 0, 166, 0, 0, 0, 201, 0, 0, 0, 188, 0, 0, 0, 222, 0, 0, 0, 209, 0, 0, 0, 244, 0, 0, 0, 230, 0, 0, 1, 7, 0, 0, 0, 252, 0, 0, 1, 27, 0, 0, 1, 15, 0, 0, 1, 47, 0, 0, 1, 35, 0, 0, 1, 73, 0, 0, 1, 68, 0, 0, 1, 80, 0, 0, 1, 78, 0, 0, 1, 87, 0, 0, 1, 82, 0, 0, 1, 92, 0, 0, 1, 89, 0, 0, 1, 98, 0, 0, 1, 71, 0, 0, 1, 101, 0, 0, 1, 66, 0, 0, 1, 126, 0, 0, 1, 114, 0, 0, 1, 133, 0, 0, 1, 128, 0, 0, 1, 140, 0, 0, 1, 138, 0, 0, 1, 147, 0, 0, 1, 142, 0, 0, 1, 152, 0, 0, 1, 149, 0, 0, 1, 158, 0, 0, 1, 131];

        let expected = vec![19, 5, 40, 27, 61, 48, 83, 69, 102, 91, 122, 110, 142, 130, 180, 166, 201, 188, 222, 209, 244, 230, 263, 252, 283, 271, 303, 291, 329, 324, 336, 334, 343, 338, 348, 345, 354, 327, 357, 322, 382, 370, 389, 384, 396, 394, 403, 398, 408, 405, 414, 387];

        let mut decoder = Decoder::new(&data);

        let Header {codec, length, parameter } = decoder.header().unwrap();
        let decoded = decoder.as_array_of_i32(length as usize).unwrap();

        assert_eq!(expected, decoded);
    }
}
