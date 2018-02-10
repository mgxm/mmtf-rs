use std::iter;
use std::i16;

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
/// use mmtf::codec::RunLength;
///
/// let encoded = [1, 4, 2, 1, 1, 4];
/// let decoded = RunLength::decode(&encoded);
/// assert_eq!(vec![1, 1, 1, 1, 2, 1, 1, 1, 1], decoded);
/// ```
pub struct RunLength;

impl RunLength {
    /// Decode and return the decoded data
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
/// use mmtf::codec::Delta;
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
/// use mmtf::codec::Integer;
///
/// let data = [1.00, 1.00, 0.50];
/// let encoded = Integer::encode(&data, 100);
/// assert_eq!(encoded, vec![100, 100, 50]);
///
/// let decoded = Integer::decode(&encoded, 100);
/// assert_eq!(decoded, data);
/// ```
pub struct Integer;

impl Integer {
    /// Decode and return the decoded data
    pub fn decode(values: &[i32], factor: i32) -> Vec<f32> {
        let result: Vec<f32> = values
            .iter()
            .map(|x| *x as f32 / factor as f32)
            .collect();
        result
    }

    /// Encode `values` with an desired `factor`
    pub fn encode(values: &[f32], factor: i32) -> Vec<i32> {
        let result: Vec<i32> = values
            .iter()
            .map(|x| (x * factor as f32) as i32)
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
/// use mmtf::codec::RecursiveIndexing;
///
/// let encoded = [1, 420, 32767, 0, 120, -32768, 0, 32767, 2];
/// let expected = vec![1, 420, 32767, 120, -32768, 32769];
///
/// let decoded = RecursiveIndexing::decode(&encoded);
/// assert_eq!(expected, decoded);
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

    #[test]
    fn test_codec_recursive_index_encoding() {
        let data = [1, 420, 32767, 0, 120, -32768, 0, 32767, 2];
        let expected = vec![1, 420, 32767, 120, -32768, 32769];
        let actual = RecursiveIndexing::decode(&data);
        assert_eq!(expected, actual);
    }
}
