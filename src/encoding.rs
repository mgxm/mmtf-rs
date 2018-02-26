use std::iter;
use std::i16;
use itertools::Itertools;
use num_integer;
use num_traits::{Float, NumCast, PrimInt};

/// Run-length encoding.
///
/// Run-length decoding can generally be used to compress arrays that contain
/// stretches of equal values. Instead of storing each value itself, stretches
/// of equal values are represented by the value itself and the occurrence count,
/// that is a value/count pair.
///
/// # Examples
///
/// ```
/// use mmtf::encoding::RunLength;
///
/// let data = [1, 1, 1, 1, 2, 1, 1, 1, 1];
/// let encoded = RunLength::encode(&data);
/// assert_eq!(vec![1, 4, 2, 1, 1, 4], encoded);
///
/// let decoded = RunLength::decode(&encoded);
/// assert_eq!(vec![1, 1, 1, 1, 2, 1, 1, 1, 1], decoded);
/// ```
pub struct RunLength;

impl RunLength {
    /// Decode and return the decoded data
    // TODO: verify if 'AsPrimitive<T>' is the better and
    // the correct way to handle generics over primitives types.
    pub fn decode<T>(bytes: &[T]) -> Vec<i32>
    where
        T: num_integer::Integer + NumCast + PrimInt,
    {
        let mut res: Vec<i32> = Vec::new();

        for v in bytes.chunks(2) {
            let value = &v[0];
            let repeat = &v[1];
            let chunks: usize = NumCast::from(*repeat).unwrap();
            for i in iter::repeat(value).take(chunks) {
                let value: i32 = NumCast::from(*i).unwrap();
                res.push(value);
            }
        }
        res
    }

    /// Encode and return the encoded data
    pub fn encode<T>(values: &[T]) -> Vec<i32>
    where
        T: num_integer::Integer + NumCast + PrimInt,
    {
        let mut result: Vec<i32> = Vec::new();

        for (key, group) in &values.into_iter().group_by(|v| *v) {
            let key: i32 = NumCast::from(*key).unwrap();
            result.push(key);
            result.push(group.count() as i32);
        }
        result
    }
}

/// Delta encoding.
///
/// Delta encoding is used to store an array of numbers. Instead of storing the
/// numbers themselves, the differences (deltas) between the numbers are stored.
/// When the values of the deltas are smaller than the numbers themselves they
/// can be more efficiently packed to require less space.
///
/// Note that arrays in which the values change by an identical amount for a range
/// of consecutive values lend themselves to subsequent run-length encoding.
///
/// # Examples
///
/// ```
/// use mmtf::encoding::Delta;
///
/// let encoded =      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 5];
/// let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 20];
///
/// let decoded = Delta::decode(&encoded);
/// assert_eq!(expected, decoded);
/// ```
pub struct Delta;

impl Delta {
    /// Decode and return the decoded data
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

    pub fn encode<T>(bytes: &[T]) -> Vec<i32>
    where
        T: num_integer::Integer + NumCast + PrimInt,
    {
        let mut buffer: Vec<i32> = Vec::with_capacity(bytes.len() as usize);

        let mut position = NumCast::from(bytes[0]).unwrap();
        buffer.push(position);
        for (index, value) in bytes.iter().skip(1).enumerate() {
            let value: i32 = NumCast::from(*value).unwrap();
            buffer.push(value - position);
            position = value;
        }
        buffer
    }
}

/// Integer encoding.
///
/// In integer encoding, floating point numbers are converted to integer values
/// by multiplying with a factor and discard everything after the decimal point.
/// Depending on the multiplication factor this can change the precision but with
/// a sufficiently large factor it is lossless. The integer values can then often
/// be compressed with delta encoding which is the main motivation for it.
///
/// # Examples
///
/// ```
/// use mmtf::encoding::IntegerEncoding;
///
/// let data = [1.00, 1.00, 0.50];
/// let encoded = IntegerEncoding::encode(&data, 100);
/// assert_eq!(encoded, vec![100, 100, 50]);
///
/// let decoded = IntegerEncoding::decode(&encoded, 100);
/// assert_eq!(decoded, data);
/// ```
pub struct IntegerEncoding;

impl IntegerEncoding {
    /// Decode and return the decoded data
    pub fn decode<T>(values: &[T], factor: i32) -> Vec<f32>
    where
        T: num_integer::Integer + NumCast + PrimInt,
    {
        let result: Vec<f32> = values
            .iter()
            .map(|x| {
                let value: f32 = NumCast::from(*x).unwrap();
                value / factor as f32
            })
            .collect();
        result
    }

    /// Encode `values` with an desired `factor`
    pub fn encode<T>(values: &[T], factor: i32) -> Vec<i32>
    where
        T: Float,
    {
        let result: Vec<i32> = values
            .iter()
            .map(|x| {
                let x: T = NumCast::from(*x).unwrap();
                let factor: T = NumCast::from(factor).unwrap();
                let result: i32 = NumCast::from(x * factor).unwrap();
                result
            })
            .collect();
        result
    }
}

/// Recursive indexing encoding

/// Recursive indexing encodes values such that the encoded values lie within the
/// open interval (MIN, MAX). This allows to create a more compact representation
/// of a 32-bit signed integer array when the majority of values in the array fit
/// into 16-bit (or 8-bit). To encode each value in the input array the method
/// stores the value itself if it lies within the open interval (MIN, MAX),
/// otherwise the MAX (or MIN if the number is negative) interval endpoint is stored
/// and subtracted from the input value. This process of storing and subtracting is
/// repeated recursively until the remainder lies within the interval.
///
/// Note that `MAX` and `MIN` are the largest and smallest value that can be
/// represented by the `i16` integer type
///
/// # Examples
///
/// ```
/// use mmtf::encoding::RecursiveIndexing;
///
/// let data = [1, 420, 32767, 120, -32768, 32769];
///
/// let encoded = RecursiveIndexing::encode(&data);
/// assert_eq!(encoded, vec![1, 420, 32767, 0, 120, -32768, 0, 32767, 2]);
///
/// let decoded = RecursiveIndexing::decode(&encoded);
/// assert_eq!(decoded, data);
/// ```
pub struct RecursiveIndexing;

impl RecursiveIndexing {
    /// Decode and return the decoded data
    pub fn decode(bytes: &[i16]) -> Vec<i32> {
        let mut output = Vec::new();
        let mut out_len: i32 = 0;

        for item in bytes {
            if *item == i16::MAX || *item == i16::MIN {
                out_len += *item as i32;
            } else {
                out_len += *item as i32;
                output.push(out_len);
                out_len = 0;
            }
        }
        output
    }

    pub fn encode(bytes: &[i32]) -> Vec<i16> {
        let mut output: Vec<i16> = Vec::new();
        for num in bytes {
            let mut num = *num;
            if num >= 0 {
                while num >= i16::MAX as i32 {
                    output.push(i16::MAX);
                    num -= i16::MAX as i32;
                }
            } else {
                while num <= i16::MIN as i32 {
                    output.push(i16::MIN);
                    num += (i16::MIN as i32).abs();
                }
            }
            output.push(num as i16);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decode_run_length_encoding() {
        let encoded = [1, 4, 2, 1, 1, 4];
        let decoded = RunLength::decode(&encoded);
        assert_eq!(vec![1, 1, 1, 1, 2, 1, 1, 1, 1], decoded);

        let encode = [1_i16, 4, 2, 1, 1, 4];
        let decoded = RunLength::decode(&encoded);
        assert_eq!(vec![1, 1, 1, 1, 2, 1, 1, 1, 1], decoded);
    }

    #[test]
    fn it_encode_run_length_encoding() {
        let encoded = [1, 1, 1, 1, 2, 1, 1, 1, 1];
        let decoded = RunLength::encode(&encoded);
        assert_eq!(vec![1, 4, 2, 1, 1, 4], decoded);
    }

    #[test]
    fn it_decode_delta_encoding() {
        let data = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 5];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 20];
        let actual = Delta::decode(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_encode_delta_encoding() {
        let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 20];
        let expected = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 5];
        let actual = Delta::encode(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_decode_integer_decoding() {
        let data = [100, 100, 100, 100, 50, 50];
        let expected = vec![1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
        let actual = IntegerEncoding::decode(&data, 100);
        assert_eq!(expected, actual);

        let data = [100_i16, 100, 100, 100, 50, 50];
        let expected = vec![1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
        let actual = IntegerEncoding::decode(&data, 100);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_encode_integer_encoding() {
        let data = [1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
        let expected = vec![100, 100, 100, 100, 50, 50];
        let actual = IntegerEncoding::encode(&data, 100);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_decode_recursive_index_encoding() {
        let data = [1, 420, 32767, 0, 120, -32768, 0, 32767, 2];
        let expected = vec![1, 420, 32767, 120, -32768, 32769];
        let actual = RecursiveIndexing::decode(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_encode_recursive_index_encoding() {
        let data = [1, 420, 32767, 120, -32768, 32769];
        let expected = vec![1, 420, 32767, 0, 120, -32768, 0, 32767, 2];
        let actual = RecursiveIndexing::encode(&data);
        assert_eq!(expected, actual);
    }
}
