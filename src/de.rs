use serde::de::{self, MapAccess};
use std::fmt;
use serde_bytes;

use super::encode::{EncodeError, Header, HeaderLayout, Strategy, StrategyDataTypes};
use super::decode::Decoder;

pub struct Deserializer<D> {
    deserializer: D,
}

impl<'de, D> Deserializer<D>
where
    D: de::Deserializer<'de>,
{
    pub fn new(deserializer: D) -> Deserializer<D> {
        Deserializer { deserializer }
    }
}

macro_rules! forward_deserialize {
        ($name:ident) => {forward_deserialize!($name, );};
        ($name:ident, $($arg:tt => $ty:ty),*) => {
            fn $name<V>(self, $($arg: $ty,)* visitor: V) -> Result<V::Value, D::Error>
            where V: de::Visitor<'de>
            {
                let visitor = Visitor {
                    visitor: visitor
                };
                self.deserializer.$name($($arg,)* visitor)
            }
        }

    }

impl<'de, D> de::Deserializer<'de> for Deserializer<D>
where
    D: de::Deserializer<'de>,
{
    type Error = D::Error;

    forward_deserialize!(deserialize_any);
    // forward_deserialize!(deserialize_bytes);

    forward_to_deserialize_any! {
        bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq map
        tuple_struct struct identifier tuple unit option byte_buf bytes
        unit_struct newtype_struct ignored_any enum
    }
}

struct Visitor<V> {
    visitor: V,
}

impl<V> Visitor<V> {
    fn decode_bytes<'a, T, E>(&self, s: &[u8]) -> Result<T, E>
    where
        T: From<StrategyDataTypes>,
        E: de::Error,
    {
        let decoded = Decoder::new(s).apply().unwrap();
        Ok(From::from(decoded))
    }
}

macro_rules! forward_visit {
        ($name:ident, $ty:ty) => (
            fn $name<E>(self, v: $ty) -> Result<V::Value, E>
            where E: de::Error
            {
                self.visitor.$name(v)
            }
        );
        ($name:ident, $bond:ident, $t:tt) => (
            fn $name<V2>(self, t: V2) -> Result<V::Value, V2::Error>
            where
                V2: $bond<$t>
            {
                self.visitor.$name(t)
            }
        );
    }

impl<'de, V> de::Visitor<'de> for Visitor<V>
where
    V: de::Visitor<'de>,
{
    type Value = V::Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.visitor.expecting(formatter)
    }

    forward_visit!(visit_bool, bool);
    forward_visit!(visit_i8, i8);
    forward_visit!(visit_i16, i16);
    forward_visit!(visit_i32, i32);
    forward_visit!(visit_i64, i64);
    forward_visit!(visit_u8, u8);
    forward_visit!(visit_u16, u16);
    forward_visit!(visit_u32, u32);
    forward_visit!(visit_u64, u64);
    forward_visit!(visit_f32, f32);
    forward_visit!(visit_f64, f64);
    forward_visit!(visit_char, char);
    // forward_visit!(visit_bytes, &[u8]);
    forward_visit!(visit_byte_buf, Vec<u8>);
    forward_visit!(visit_map, MapAccess, 'de);

    fn visit_bytes<E>(self, v: &[u8]) -> Result<V::Value, E>
    where
        E: de::Error,
    {
        let decoded = self.decode_bytes(v);
        Ok(decoded)
    }
}
