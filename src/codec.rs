use std::iter;

struct RunLength;

impl RunLength {
    fn decode(bytes: &[i32]) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        for v in bytes.chunks(2) {
            let value = &v[0];
            let repeat = &v[1];
            for i in iter::repeat(value).take(*repeat as usize) {
                res.push(*i);
            }
        }
        res
    }
}

struct Delta;

impl Delta {
    fn decode(bytes: &[i32]) -> Vec<i32> {
        let mut buffer = Vec::with_capacity(bytes.len() as usize);

        // The first entry in the array is left as is
        buffer.push(bytes[0]);

        for (index, value) in bytes.iter().skip(1).enumerate() {
            let position = buffer[index];
            buffer.push(position + value)
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decode_run_length() {
        let encoded = [1, 4, 2, 1, 1, 4];
        let decoded = RunLength::decode(&encoded);
        assert_eq!(vec![1,1,1,1,2,1,1,1,1], decoded);
    }

    #[test]
    fn test_codec_delta_encoding() {
        let data = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 5];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 20];
        let actual = Delta::decode(&data);
        assert_eq!(expected, actual);
    }
}
