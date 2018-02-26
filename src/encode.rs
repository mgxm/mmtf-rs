use std::convert::From;

#[derive(Debug)]
/// Map msgpack object into the given targets.
pub enum StrategyDataTypes {
    /// strategies: 1, 9, 10, 11, 12, 13
    VecFloat32(Vec<f32>),
    /// strategies: 2
    VecInt8(Vec<i8>),
    /// strategies: 3
    VecInt16(Vec<i16>),
    /// strategies: 4, 7, 8, 14, 15
    VecInt32(Vec<i32>),
    /// strategies: 5
    VecString(Vec<String>),
    /// strategies: 6
    VecChar(Vec<char>),
}

impl From<StrategyDataTypes> for Option<Vec<i8>> {
    fn from(value: StrategyDataTypes) -> Option<Vec<i8>> {
        Some(From::from(value))
    }
}

impl From<StrategyDataTypes> for Option<Vec<char>> {
    fn from(value: StrategyDataTypes) -> Option<Vec<char>> {
        Some(From::from(value))
    }
}

impl From<StrategyDataTypes> for Option<Vec<String>> {
    fn from(value: StrategyDataTypes) -> Option<Vec<String>> {
        Some(From::from(value))
    }
}

impl From<StrategyDataTypes> for Option<Vec<i32>> {
    fn from(value: StrategyDataTypes) -> Option<Vec<i32>> {
        Some(From::from(value))
    }
}

impl From<StrategyDataTypes> for Option<Vec<f32>> {
    fn from(value: StrategyDataTypes) -> Option<Vec<f32>> {
        Some(From::from(value))
    }
}

impl From<StrategyDataTypes> for Vec<char> {
    fn from(value: StrategyDataTypes) -> Vec<char> {
        match value {
            StrategyDataTypes::VecChar(some) => some,
            _ => unreachable!(),
        }
    }
}

impl From<StrategyDataTypes> for Vec<String> {
    fn from(value: StrategyDataTypes) -> Vec<String> {
        match value {
            StrategyDataTypes::VecString(some) => some,
            _ => unreachable!(),
        }
    }
}

impl From<StrategyDataTypes> for Vec<i16> {
    fn from(value: StrategyDataTypes) -> Vec<i16> {
        match value {
            StrategyDataTypes::VecInt16(some) => some,
            _ => unreachable!(),
        }
    }
}

impl From<StrategyDataTypes> for Vec<i32> {
    fn from(value: StrategyDataTypes) -> Vec<i32> {
        match value {
            StrategyDataTypes::VecInt32(some) => some,
            _ => unreachable!(),
        }
    }
}

impl From<StrategyDataTypes> for Vec<f32> {
    fn from(value: StrategyDataTypes) -> Vec<f32> {
        match value {
            StrategyDataTypes::VecFloat32(some) => some,
            _ => unreachable!(),
        }
    }
}

impl From<StrategyDataTypes> for Vec<i8> {
    fn from(value: StrategyDataTypes) -> Vec<i8> {
        match value {
            StrategyDataTypes::VecInt8(some) => some,
            _ => unreachable!(),
        }
    }
}

pub trait Strategy {
    fn apply(&mut self) -> Result<StrategyDataTypes, &'static str>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_convert_to_vec_i8() {
        let data = StrategyDataTypes::VecInt8(vec![0_i8]);
        let result: Vec<i8> = From::from(data);
        assert_eq!(vec![0_i8], result);

        let data = StrategyDataTypes::VecInt8(vec![0_i8]);
        let result: Option<Vec<i8>> = From::from(data);
        assert_eq!(Some(vec![0_i8]), result);
    }

    #[test]
    fn it_convert_to_vec_i16() {
        let data = StrategyDataTypes::VecInt16(vec![0_i16]);
        let result: Vec<i16> = From::from(data);
        assert_eq!(vec![0_i16], result);
    }

    #[test]
    fn it_convert_to_vec_i32() {
        let data = StrategyDataTypes::VecInt32(vec![0]);
        let result: Vec<i32> = From::from(data);
        assert_eq!(vec![0], result);

        let data = StrategyDataTypes::VecInt32(vec![0]);
        let result: Option<Vec<i32>> = From::from(data);
        assert_eq!(Some(vec![0]), result);
    }

    #[test]
    fn it_convert_to_vec_f32() {
        let data = StrategyDataTypes::VecFloat32(vec![0.0]);
        let result: Vec<f32> = From::from(data);
        assert_eq!(vec![0.0], result);

        let data = StrategyDataTypes::VecFloat32(vec![0.0]);
        let result: Option<Vec<f32>> = From::from(data);
        assert_eq!(Some(vec![0.0]), result);
    }

    #[test]
    fn it_convert_to_vec_char() {
        let data = StrategyDataTypes::VecChar(vec!['a']);
        let result: Vec<char> = From::from(data);
        assert_eq!(vec!['a'], result);

        let data = StrategyDataTypes::VecChar(vec!['a']);
        let result: Option<Vec<char>> = From::from(data);
        assert_eq!(Some(vec!['a']), result);
    }

    #[test]
    fn it_convert_to_vec_string() {
        let data = StrategyDataTypes::VecString(vec!["A".to_string()]);
        let result: Vec<String> = From::from(data);
        assert_eq!(vec!["A".to_string()], result);
    }
}
