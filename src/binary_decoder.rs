use safe_transmute::guarded_transmute_many;
use std::char::from_u32;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

pub fn interpret_bytes_as_char(bytes: &[u8], lenght: usize, chunk_size: usize) -> Vec<char> {
    let mut buffer: Vec<char> = Vec::with_capacity(lenght);
    for c in bytes.chunks(chunk_size) {
        unsafe {
            let result = guarded_transmute_many::<u32>(&c).unwrap();
            buffer.push(from_u32(result[0]).unwrap());
        }
    }
    assert!(buffer.len() == lenght);
    buffer
}

pub fn interpret_bytes_as_i32(bytes: &[u8]) -> Vec<i32> {
    let length = bytes.len();

    assert!(length % 4 == 0);

    let mut bytes = Cursor::new(bytes);
    let mut buffer = Vec::with_capacity(length);

    for b in 0..length/4 {
        let r = bytes.read_i32::<BigEndian>().unwrap();
        buffer.push(r);
    }
    buffer
}

pub fn interpret_bytes_as_i8(bytes: &[u8], lenght: usize) -> Vec<i8> {
    let mut bytes = Cursor::new(bytes);
    let mut buffer = Vec::new();
    for b in 0..lenght {
        let r = bytes.read_i8().unwrap();
        buffer.push(r);
    }
    assert!(buffer.len() == lenght);
    buffer
}

pub fn interpret_bytes_as_i16(bytes: &[u8], lenght: usize) -> Vec<i16> {
    let mut bytes = Cursor::new(bytes);
    let mut buffer = Vec::new();
    for b in 0..lenght {
        let r = bytes.read_i16::<BigEndian>().unwrap();
        buffer.push(r);
    }
    assert!(buffer.len() == lenght);
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_bytes_as_char() {
        let data = [65, 0, 0, 0, 66, 0, 0, 0, 67, 0, 0, 0];
        let expected = vec!['A', 'B', 'C'];
        let actual = interpret_bytes_as_char(&data, 3 as usize, 4 as usize);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_interpret_bytes_as_i32() {
        let data = [0, 0, 0, 19, 0, 0, 0, 5, 0, 0, 0, 40];
        let expected = vec![19, 5, 40];
        let actual = interpret_bytes_as_i32(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_interpret_bytes_as_i8() {
        let data = [1, 1, 1];
        let expected = vec![1, 1, 1];
        let actual = interpret_bytes_as_i8(&data, 3 as usize);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_interpret_bytes_as_i16() {
        let data = [0, 10, 0, 20, 0, 22];
        let expected = vec![10, 20, 22];
        let actual = interpret_bytes_as_i16(&data, 3 as usize);
        assert_eq!(expected, actual);
    }
}
