use std::str::from_utf8;

use serde::{
    de::{IgnoredAny, Visitor},
    Deserialize,
};

struct NumberSM {
    sign: bool,
    num: usize,
}
impl NumberSM {
    fn new_unsigned(ch: u8) -> Self {
        Self {
            sign: false,
            num: (ch - b'0') as _,
        }
    }
    fn new_signed() -> Self {
        Self { sign: true, num: 0 }
    }
    fn accumulate(self, ch: u8) -> Self {
        Self {
            sign: self.sign,
            num: self.num * 10 + (ch - b'0') as usize,
        }
    }

    fn finish(self) -> isize {
        (if self.sign { -1 } else { 1 }) * self.num as isize
    }
}

pub fn part1(input: &str) -> isize {
    let mut total = 0;
    let mut accumulate = None;
    for b in input.bytes() {
        accumulate = match (accumulate, b) {
            (None, b'0'..=b'9') => Some(NumberSM::new_unsigned(b)),
            (None, b'-') => Some(NumberSM::new_signed()),
            (None, _) => None,
            (Some(accumulate), b'0'..=b'9') => Some(accumulate.accumulate(b)),
            (Some(accumulate), b'-') => {
                total += accumulate.finish();
                Some(NumberSM::new_signed())
            }
            (Some(accumulate), _) => {
                total += accumulate.finish();
                None
            }
        }
    }
    total + accumulate.map(NumberSM::finish).unwrap_or(0)
}

struct NumberCount(isize);
impl<'de> Deserialize<'de> for NumberCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum RedOrNumberCount {
            Red,
            NumberCount(NumberCount),
        }
        impl<'de> Deserialize<'de> for RedOrNumberCount {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct RedOrNumberVisitor;
                impl<'de> Visitor<'de> for RedOrNumberVisitor {
                    type Value = RedOrNumberCount;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("\"red\" or any json")
                    }

                    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_bool(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor.visit_i8(v).map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_i16<E>(self, v: i16) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_i16(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_i32(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_i64(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_i128(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor.visit_u8(v).map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_u16<E>(self, v: u16) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_u16(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_u32(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_u64(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_u128(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_f32<E>(self, v: f32) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_f32(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_f64(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_char<E>(self, v: char) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_char(v)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "red" => Ok(RedOrNumberCount::Red),
                            _ => NumberVisitor
                                .visit_str(v)
                                .map(RedOrNumberCount::NumberCount),
                        }
                    }

                    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            "red" => Ok(RedOrNumberCount::Red),
                            _ => NumberVisitor
                                .visit_borrowed_str(v)
                                .map(RedOrNumberCount::NumberCount),
                        }
                    }

                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v.as_str() {
                            "red" => Ok(RedOrNumberCount::Red),
                            _ => NumberVisitor
                                .visit_string(v)
                                .map(RedOrNumberCount::NumberCount),
                        }
                    }

                    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            b"red" => Ok(RedOrNumberCount::Red),
                            _ => NumberVisitor
                                .visit_bytes(v)
                                .map(RedOrNumberCount::NumberCount),
                        }
                    }

                    fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v {
                            b"red" => Ok(RedOrNumberCount::Red),
                            _ => NumberVisitor
                                .visit_borrowed_bytes(v)
                                .map(RedOrNumberCount::NumberCount),
                        }
                    }

                    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match v.as_slice() {
                            b"red" => Ok(RedOrNumberCount::Red),
                            _ => NumberVisitor
                                .visit_byte_buf(v)
                                .map(RedOrNumberCount::NumberCount),
                        }
                    }

                    fn visit_none<E>(self) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_none()
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        NumberVisitor
                            .visit_some(deserializer)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_unit<E>(self) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        NumberVisitor
                            .visit_unit()
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_newtype_struct<D>(
                        self,
                        deserializer: D,
                    ) -> Result<Self::Value, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        NumberVisitor
                            .visit_newtype_struct(deserializer)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        NumberVisitor
                            .visit_seq(seq)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::MapAccess<'de>,
                    {
                        NumberVisitor
                            .visit_map(map)
                            .map(RedOrNumberCount::NumberCount)
                    }

                    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::EnumAccess<'de>,
                    {
                        NumberVisitor
                            .visit_enum(data)
                            .map(RedOrNumberCount::NumberCount)
                    }
                }

                deserializer.deserialize_any(RedOrNumberVisitor)
            }
        }

        struct NumberVisitor;
        impl<'de> Visitor<'de> for NumberVisitor {
            type Value = NumberCount;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("any json value")
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let _ = v;
                Ok(NumberCount(0))
            }

            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberCount(v as _))
            }

            fn visit_i128<E>(self, v: i128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberCount(v as _))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberCount(v as _))
            }

            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberCount(v as _))
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let _ = v;
                Ok(NumberCount(0))
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let _ = v;
                Ok(NumberCount(0))
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberCount(0))
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(NumberVisitor)
            }

            fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(NumberCount(0))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                deserializer.deserialize_any(NumberVisitor)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut total = 0;
                while let Some(v) = seq.next_element::<NumberCount>()? {
                    total += v.0;
                }
                Ok(NumberCount(total))
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut total = 0;
                while let Some((_, v)) = map.next_entry::<IgnoredAny, RedOrNumberCount>()? {
                    match v {
                        RedOrNumberCount::Red => {
                            // consume the rest of the map
                            while let Some((_, _)) = map.next_entry::<IgnoredAny, IgnoredAny>()? {}
                            // return 0
                            return Ok(NumberCount(0));
                        }
                        RedOrNumberCount::NumberCount(NumberCount(v)) => total += v,
                    }
                }
                Ok(NumberCount(total))
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}

pub fn part2(input: &str) -> isize {
    serde_json::from_str::<NumberCount>(input).unwrap().0
}
