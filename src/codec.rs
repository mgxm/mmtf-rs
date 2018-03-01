use num_integer::Integer;
use num_traits::{Float, NumCast, PrimInt, ToPrimitive};
use binary_decoder::Interpret;
use binary_decoder;
use encoding::{Delta, IntegerEncoding, RecursiveIndexing, RunLength};
use encode::EncodeError;

/// Delta & Runlength
///
/// Interpret bytes as array of 32-bit signed integers,
/// then run-length decode into array of 32-bit signed integers,
/// then delta decode into array of 32-bit signed integers.
///
/// # Examples
///
/// ```
/// use mmtf::codec::DeltaRunlength;
///
/// let data = [1, 2, 3, 4];
/// let encoded = DeltaRunlength::encode(&data);
/// assert_eq!(encoded, vec![0, 0, 0, 1, 0, 0, 0, 4]);
///
/// let decoded = DeltaRunlength::decode(&encoded).unwrap();
/// assert_eq!(decoded, data);
/// ```
#[derive(Debug)]
pub struct DeltaRunlength;

impl DeltaRunlength {
    /// Decode given bytes
    pub fn decode(bytes: &[u8]) -> Result<Vec<i32>, EncodeError> {
        let asi32: Vec<i32> = binary_decoder::Interpret::from(bytes);
        let runlen = try!(RunLength::decode(&asi32));
        Ok(Delta::decode(&runlen))
    }

    /// Encode any array of 'T' where `T ` can be any Integer.
    pub fn encode<T>(value: &[T]) -> Vec<u8>
    where
        T: Integer + NumCast + PrimInt + ToPrimitive,
    {
        let delta = Delta::encode(value);
        let runlen = RunLength::encode(&delta);
        let result: Vec<u8> = Interpret::from(&runlen[..]);
        result
    }
}

/// Integer & Runlength encoded 32-bit floating-point number array
///
/// Interpret bytes as array of 32-bit signed integers,
/// then run-length decode into array of 32-bit signed integers, then
/// integer decode into array of 32-bit floating-point numbers using
/// the divisor parameter.
///
/// # Examples
///
/// ```
/// use mmtf::codec::IntegerRunLength;
///
/// let data = [1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
/// let encoded = IntegerRunLength::encode(&data, 100);
/// assert_eq!(encoded, vec![0, 0, 0, 100, 0, 0, 0, 4, 0, 0, 0, 50, 0, 0, 0, 2]);
///
/// let decoded = IntegerRunLength::decode(&encoded, 100).unwrap();
/// assert_eq!(decoded, data);
/// ```
#[derive(Debug)]
pub struct IntegerRunLength;

impl IntegerRunLength {
    /// Decode given bytes
    pub fn decode(bytes: &[u8], factor: i32) -> Result<Vec<f32>, EncodeError> {
        let asi32: Vec<i32> = binary_decoder::Interpret::from(bytes);
        let runlen = try!(RunLength::decode(&asi32));
        Ok(IntegerEncoding::decode(&runlen, factor))
    }

    /// Encode any array of 'T' where `T ` can be any Float.
    pub fn encode<T>(value: &[T], factor: i32) -> Vec<u8>
    where
        T: Float + NumCast,
    {
        let integer = IntegerEncoding::encode(value, factor);
        let runlen = RunLength::encode(&integer);
        let result: Vec<u8> = Interpret::from(&runlen[..]);
        result
    }
}

/// Integer & delta encoded & two-byte-packed 32-bit floating-point number array
///
/// Description Interpret bytes as array of 16-bit signed integers, then unpack
/// into array of 32-bit integers, then delta decode into array of 32-bit integers,
/// then integer decode into array of 32-bit floating-point numbers using the divisor
/// parameter.
///
/// # Examples
///
/// ```
/// use mmtf::codec::IntegerDeltaRecursive;
///
/// let data = [182.00, 182.00, 182.03];
/// let encoded = IntegerDeltaRecursive::encode(&data, 100);
/// assert_eq!(encoded, vec![71, 24, 0, 0, 0, 3]);
///
/// let decoded = IntegerDeltaRecursive::decode(&encoded, 100);
/// assert_eq!(decoded, data);
/// ```
#[derive(Debug)]
pub struct IntegerDeltaRecursive;

impl IntegerDeltaRecursive {
    /// Decode given bytes
    pub fn decode(bytes: &[u8], factor: i32) -> Vec<f32> {
        let asi16: Vec<i16> = binary_decoder::Interpret::from(bytes);
        let recursive = RecursiveIndexing::decode(&asi16);
        let delta = Delta::decode(&recursive);
        IntegerEncoding::decode(&delta, factor)
    }

    /// Encode any array of 'T' where `T ` can be any Float.
    pub fn encode<T>(value: &[T], factor: i32) -> Vec<u8>
    where
        T: Float + NumCast,
    {
        let integer = IntegerEncoding::encode(value, factor);
        let delta = Delta::encode(&integer);
        let recursive = RecursiveIndexing::encode(&delta);
        let result: Vec<u8> = Interpret::from(&recursive[..]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decode_delta_run_length() {
        let data = [
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 5, 255, 255, 255, 245, 0, 0, 0, 1,
        ];
        let expected = vec![0, 1, 2, 3, 5, 5, 6, 7, 8, 9, 10, -1];
        let actual = DeltaRunlength::decode(&data).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_encode_delta_run_length() {
        let data = [0, 1, 2, 3, 5, 5, 6, 7, 8, 9, 10, -1];
        let expected = vec![
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 3, 0, 0, 0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 5, 255, 255, 255, 245, 0, 0, 0, 1,
        ];
        let actual = DeltaRunlength::encode(&data);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_decode_integer_delta_run_length() {
        let data = [0, 0, 0, 100, 0, 0, 0, 4, 0, 0, 0, 50, 0, 0, 0, 2];
        let expected = vec![1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
        let actual = IntegerRunLength::decode(&data, 100).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_encode_integer_delta_run_length() {
        let data = [1.00, 1.00, 1.00, 1.00, 0.50, 0.50];
        let expected = vec![0, 0, 0, 100, 0, 0, 0, 4, 0, 0, 0, 50, 0, 0, 0, 2];
        let actual = IntegerRunLength::encode(&data, 100);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_decode_integer_delta_recursive() {
        let data = [71, 24, 0, 0, 0, 2, 255, 255, 0, 100, 255, 253, 0, 5];
        let expected = vec![182.00, 182.00, 182.02, 182.01, 183.01, 182.98, 183.03];
        let actual = IntegerDeltaRecursive::decode(&data, 100);
        assert_eq!(expected, actual);
    }

    #[test]
    fn it_encode_integer_delta_recursive() {
        let data = [182.00, 182.00, 182.02, 182.01, 183.01, 182.98, 183.03];
        let expected = vec![71, 24, 0, 0, 0, 2, 255, 255, 0, 100, 255, 253, 0, 5];
        let actual = IntegerDeltaRecursive::encode(&data, 100);
        assert_eq!(expected, actual);
    }
}
