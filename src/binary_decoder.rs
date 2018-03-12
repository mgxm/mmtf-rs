use std::char::from_u32;
use std::str;
use std;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Cursor;
use std::io::Error;

pub trait Interpret<T> {
    fn from(T) -> Result<Self, Error>
    where
        Self: std::marker::Sized;
}

impl<'a> Interpret<&'a [i32]> for Vec<u8> {
    fn from(values: &'a [i32]) -> Result<Vec<u8>, Error> {
        let mut wtr = vec![];
        for v in values {
            wtr.write_i32::<BigEndian>(*v)?;
        }
        Ok(wtr)
    }
}

impl<'a> Interpret<&'a [i16]> for Vec<u8> {
    fn from(values: &'a [i16]) -> Result<Vec<u8>, Error> {
        let mut wtr = vec![];
        for v in values {
            wtr.write_i16::<BigEndian>(*v)?;
        }
        Ok(wtr)
    }
}

impl<'a> Interpret<&'a [u8]> for Vec<char> {
    fn from(values: &'a [u8]) -> Result<Vec<char>, Error> {
        let length = values.len();

        assert!(length % 4 == 0);

        let mut buffer: Vec<char> = Vec::with_capacity(length / 4);
        let mut rdr = Cursor::new(values);

        for _ in 0..length / 4 {
            let c = rdr.read_u32::<LittleEndian>()?;
            buffer.push(from_u32(c).unwrap());
        }

        Ok(buffer)
    }
}

impl<'a> Interpret<&'a [i32]> for Vec<char> {
    fn from(values: &'a [i32]) -> Result<Vec<char>, Error> {
        let length = values.len();

        let mut buffer: Vec<char> = Vec::with_capacity(length / 4);

        for c in values {
            buffer.push(from_u32(*c as u32).unwrap());
        }

        Ok(buffer)
    }
}

impl<'a> Interpret<&'a [u8]> for Vec<String> {
    fn from(values: &'a [u8]) -> Result<Vec<String>, Error> {
        let length = values.len();

        assert!(length % 4 == 0);

        let mut buffer: Vec<String> = Vec::with_capacity(length / 4);

        for c in values.chunks(4) {
            let out = str::from_utf8(c).unwrap();
            buffer.push(out.trim_matches('\u{0}').to_string());
        }

        Ok(buffer)
    }
}

impl<'a> Interpret<&'a [u8]> for Vec<f32> {
    fn from(values: &'a [u8]) -> Result<Vec<f32>, Error> {
        let length = values.len();

        assert!(length % 4 == 0);

        let mut bytes = Cursor::new(values);
        let mut buffer = Vec::with_capacity(length);

        for _ in 0..length / 4 {
            let r = bytes.read_f32::<BigEndian>()?;
            buffer.push(r);
        }
        Ok(buffer)
    }
}

impl<'a> Interpret<&'a [u8]> for Vec<i32> {
    fn from(values: &'a [u8]) -> Result<Vec<i32>, Error> {
        let length = values.len();

        assert!(length % 4 == 0);

        let mut bytes = Cursor::new(values);
        let mut buffer = Vec::with_capacity(length);

        for _ in 0..length / 4 {
            let r = bytes.read_i32::<BigEndian>()?;
            buffer.push(r);
        }
        Ok(buffer)
    }
}

impl<'a> Interpret<&'a [u8]> for Vec<i8> {
    fn from(values: &'a [u8]) -> Result<Vec<i8>, Error> {
        let length = values.len();

        let mut bytes = Cursor::new(values);
        let mut buffer = Vec::new();

        for _ in 0..length {
            let r = bytes.read_i8()?;
            buffer.push(r);
        }
        assert!(buffer.len() == length);
        Ok(buffer)
    }
}

impl<'a> Interpret<&'a [u8]> for Vec<i16> {
    fn from(values: &'a [u8]) -> Result<Vec<i16>, Error> {
        let length = values.len();

        assert!(length % 2 == 0);

        let mut bytes = Cursor::new(values);
        let mut buffer = Vec::with_capacity(length);
        for _ in 0..length / 2 {
            let r = bytes.read_i16::<BigEndian>()?;
            buffer.push(r);
        }
        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_interpret_bytes_as_char() {
        let data = [65_u8, 0, 0, 0, 66, 0, 0, 0, 67, 0, 0, 0, 68, 0, 0, 0];
        let expected = vec!['A', 'B', 'C', 'D'];
        let actual: Vec<char> = Interpret::from(&data[..]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_interpret_bytes_as_string() {
        let data = [65, 0, 0, 0, 68, 65, 0, 0];
        let expected = vec!["A", "DA"];
        let actual: Vec<String> = Interpret::from(&data[..]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_interpret_bytes_as_f32() {
        let data = [63, 153, 153, 154, 64, 57, 153, 154];
        let expected = vec![1.2, 2.9];
        let actual: Vec<f32> = Interpret::from(&data[..]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_interpret_array_of_u8_from_array_of_i32() {
        let data = [2, 3, 3];
        let expected = vec![0_u8, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 3];
        let actual: Vec<u8> = Interpret::from(&data[..]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_interpret_bytes_as_i32() {
        let data = [0, 0, 0, 19, 0, 0, 0, 5, 0, 0, 0, 40];
        let expected = vec![19, 5, 40];
        let actual: Vec<i32> = Interpret::from(&data[..]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_interpret_bytes_as_i8() {
        let data = [1, 1, 1];
        let expected = vec![1, 1, 1];
        let actual: Vec<i8> = Interpret::from(&data[..]).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_interpret_bytes_as_i16() {
        let data = [0, 10, 0, 20, 0, 22];
        let expected = vec![10, 20, 22];
        let actual: Vec<i16> = Interpret::from(&data[..]).unwrap();
        assert_eq!(expected, actual);
    }
}
