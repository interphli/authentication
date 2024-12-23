use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Serialize, Deserialize};
use std::fmt::{Formatter, Display};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum Number {
    U128(u128),
    I128(i128),
    U64(u64),
    I64(i64),
    F64(f64),
    U32(u32),
    I32(i32),
    F32(f32),
    U16(u16),
    I16(i16),
    U8(u8),
    I8(i8)
}


impl From<Number> for u128 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<u128> for Number {
    fn from(value: u128) -> Self {
        Number::U128(value)
    }
}


impl From<Number> for i128 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<i128> for Number {
    fn from(value: i128) -> Self {
        Number::I128(value)
    }
}


impl From<Number> for u64 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<u64> for Number {
    fn from(value: u64) -> Self {
        Number::U64(value)
    }
}


impl From<Number> for i64 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Number::I64(value)
    }
}


impl From<Number> for f64 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Number::F64(value)
    }
}


impl From<Number> for u32 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<u32> for Number {
    fn from(value: u32) -> Self {
        Number::U32(value)
    }
}


impl From<Number> for i32 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number::I32(value)
    }
}


impl From<Number> for f32 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<f32> for Number {
    fn from(value: f32) -> Self {
        Number::F32(value)
    }
}


impl From<Number> for u16 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<u16> for Number {
    fn from(value: u16) -> Self {
        Number::U16(value)
    }
}


impl From<Number> for i16 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<i16> for Number {
    fn from(value: i16) -> Self {
        Number::I16(value)
    }
}


impl From<Number> for u8 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<u8> for Number {
    fn from(value: u8) -> Self {
        Number::U8(value)
    }
}


impl From<Number> for i8 {
    fn from(number: Number) -> Self {
        match number {
            Number::U128(number) => number as Self,
            Number::I128(number) => number as Self,
            Number::U64(number) => number as Self,
            Number::I64(number) => number as Self,
            Number::F64(number) => number as Self,
            Number::U32(number) => number as Self,
            Number::I32(number) => number as Self,
            Number::F32(number) => number as Self,
            Number::U16(number) => number as Self,
            Number::I16(number) => number as Self,
            Number::U8(number) => number as Self,
            Number::I8(number) => number as Self
        }
    }
}

impl From<i8> for Number {
    fn from(value: i8) -> Self {
        Number::I8(value)
    }
}


impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::U128(number) => write!(f, "{number}"),
            Number::I128(number) => write!(f, "{number}"),
            Number::U64(number) => write!(f, "{number}"),
            Number::I64(number) => write!(f, "{number}"),
            Number::F64(number) => write!(f, "{number}"),
            Number::U32(number) => write!(f, "{number}"),
            Number::I32(number) => write!(f, "{number}"),
            Number::F32(number) => write!(f, "{number}"),
            Number::U16(number) => write!(f, "{number}"),
            Number::I16(number) => write!(f, "{number}"),
            Number::U8(number) => write!(f, "{number}"),
            Number::I8(number) => write!(f, "{number}")
        }
    }
}


impl From<Number> for AttributeValue {
    fn from(number: Number) -> Self {
        let number = number.to_string();
        AttributeValue::N(number)
    }
}