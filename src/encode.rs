use std::convert::From;
use std::fmt;
use std::io;

use rdir_encoding::RdirError;

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

pub trait Strategy {
    fn apply(&mut self) -> Result<StrategyDataTypes, EncodeError>;
}

#[derive(Debug)]
pub struct HeaderLayout {
    pub codec: i32,
    pub length: i32,
    pub parameter: i32,
}

pub trait Header {
    fn header(&mut self) -> Result<HeaderLayout, EncodeError>;
}

#[derive(Debug)]
pub enum EncodeError {
    Codec(String),
    Header(String),
    Field,
    Encoding(RdirError),
    IO(io::Error)
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EncodeError::Codec(ref err) => write!(f, "Codec type `{}` doesn't exists", err),
            EncodeError::Header(ref err) => write!(f, "Failed to parse Header: `{}`", err),
            EncodeError::Field => write!(f, "Failed to parse Fields"),
            EncodeError::Encoding(ref err) => write!(f, "encoding error: `{}`", err),
            EncodeError::IO(ref err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for EncodeError {
    fn from(error: io::Error) -> Self {
        EncodeError::IO(error)
    }
}

impl From<RdirError> for EncodeError {
    fn from(error: RdirError) -> Self {
        EncodeError::Encoding(error)
    }
}

macro_rules! from_strategy_for {
    ($p:path, $type:ty) => {
        impl From<StrategyDataTypes> for $type {
            fn from(value: StrategyDataTypes) -> $type {
                match value {
                    $p(some) => some,
                    _ => unreachable!()
                }
            }
        }
        impl From<StrategyDataTypes> for Option<$type> {
            fn from(value: StrategyDataTypes) -> Option<$type> {
                Some(From::from(value))
            }
        }
    }
}

from_strategy_for!(StrategyDataTypes::VecChar, Vec<char>);
from_strategy_for!(StrategyDataTypes::VecString, Vec<String>);
from_strategy_for!(StrategyDataTypes::VecInt16, Vec<i16>);
from_strategy_for!(StrategyDataTypes::VecInt32, Vec<i32>);
from_strategy_for!(StrategyDataTypes::VecFloat32, Vec<f32>);
from_strategy_for!(StrategyDataTypes::VecInt8, Vec<i8>);


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
