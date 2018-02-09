use std::iter;

pub struct RunLength;

impl RunLength {
    pub fn decode(bytes: &[i32]) -> Vec<i32> {
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

pub struct Delta;

impl Delta {
    pub fn decode(bytes: &[i32]) -> Vec<i32> {
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

pub struct Integer;

impl Integer {
    fn decode(values: &[i32], factor: i32) -> Vec<f32> {
        let result: Vec<f32> = values
            .iter()
            .map(|x| *x as f32 / factor as f32)
            .collect();
        result
    }

    fn encode(values: &[f32], factor: i32) -> Vec<i32> {
        let result: Vec<i32> = values
            .iter()
            .map(|x| (x * factor as f32) as i32)
            .collect();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decode_run_length() {
        let encoded = [1, 4, 2, 1, 1, 4];
        let decoded = RunLength::decode(&encoded);
        assert_eq!(vec![1, 1, 1, 1, 2, 1, 1, 1, 1], decoded);
    }

    #[test]
    fn test_codec_delta_encoding() {
        let data = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 5];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 20];
        let actual = Delta::decode(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_codec_integer_decoding() {
        let data = [100, 100, 100, 100, 50, 50];
        let expected = vec![1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
        let actual = Integer::decode(&data, 100);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_codec_integer_encoding() {
        let data = [1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
        let expected = vec![100, 100, 100, 100, 50, 50];
        let actual = Integer::encode(&data, 100);
        assert_eq!(expected, actual);
    }
}
