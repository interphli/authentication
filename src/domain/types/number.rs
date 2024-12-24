use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Serialize, Deserialize};
use std::fmt::{Formatter, Display};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Number {
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    F32(f32),
    U64(u64),
    I64(i64),
    F64(f64),
    U128(u128),
    I128(i128),
}


impl Number {
    pub fn attribute_value(values: &Vec<Number>) -> AttributeValue {
        let is_all_u8 = values.iter().all(|v| matches!(v, Number::U8(_)));

        if is_all_u8 {
            // Convert to a blob
            let blob: Vec<u8> = values.iter().map(|v| {
                if let Number::U8(num) = v {
                    *num
                } else {
                    0 // This should never happen due to the all check
                }
            }).collect();
            AttributeValue::B(blob.into())
        } else {
            // Convert to a number set
            let number_set: Vec<String> = values.iter().map(|v| v.to_string()).collect();
            AttributeValue::Ns(number_set)
        }
    }
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



#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_dynamodb::types::AttributeValue;

    #[test]
    fn test_number_to_u128() {
        assert_eq!(u128::from(Number::U8(255)), 255);
        assert_eq!(u128::from(Number::I8(-1)), u128::MAX);
        assert_eq!(u128::from(Number::U16(65535)), 65535);
        assert_eq!(u128::from(Number::I16(-1)), u128::MAX);
        assert_eq!(u128::from(Number::U32(4294967295)), 4294967295);
        assert_eq!(u128::from(Number::I32(-1)), u128::MAX);
        assert_eq!(u128::from(Number::U64(18446744073709551615)), 18446744073709551615);
        assert_eq!(u128::from(Number::I64(-1)), u128::MAX);
        assert_eq!(u128::from(Number::U128(340282366920938463463374607431768211455)), 340282366920938463463374607431768211455);
        assert_eq!(u128::from(Number::I128(-1)), u128::MAX);
    }

    #[test]
    fn test_number_to_i128() {
        assert_eq!(i128::from(Number::U8(255)), 255);
        assert_eq!(i128::from(Number::I8(-1)), -1);
        assert_eq!(i128::from(Number::U16(65535)), 65535);
        assert_eq!(i128::from(Number::I16(-1)), -1);
        assert_eq!(i128::from(Number::U32(4294967295)), 4294967295);
        assert_eq!(i128::from(Number::I32(-1)), -1);
        assert_eq!(i128::from(Number::U64(18446744073709551615)), 18446744073709551615);
        assert_eq!(i128::from(Number::I64(-1)), -1);
        assert_eq!(i128::from(Number::U128(340282366920938463463374607431768211455)), 340282366920938463463374607431768211455u128 as i128);
        assert_eq!(i128::from(Number::I128(-1)), -1);
    }

    #[test]
    fn test_number_to_u64() {
        assert_eq!(u64::from(Number::U8(255)), 255);
        assert_eq!(u64::from(Number::I8(-1)), u64::MAX);
        assert_eq!(u64::from(Number::U16(65535)), 65535);
        assert_eq!(u64::from(Number::I16(-1)), u64::MAX);
        assert_eq!(u64::from(Number::U32(4294967295)), 4294967295);
        assert_eq!(u64::from(Number::I32(-1)), u64::MAX);
        assert_eq!(u64::from(Number::U64(18446744073709551615)), 18446744073709551615);
        assert_eq!(u64::from(Number::I64(-1)), u64::MAX);
    }

    #[test]
    fn test_number_to_i64() {
        assert_eq!(i64::from(Number::U8(255)), 255);
        assert_eq!(i64::from(Number::I8(-1)), -1);
        assert_eq!(i64::from(Number::U16(65535)), 65535);
        assert_eq!(i64::from(Number::I16(-1)), -1);
        assert_eq!(i64::from(Number::U32(4294967295)), 4294967295);
        assert_eq!(i64::from(Number::I32(-1)), -1);
        assert_eq!(i64::from(Number::U64(18446744073709551615)), 18446744073709551615u64 as i64);
        assert_eq!(i64::from(Number::I64(-1)), -1);
    }

    #[test]
    fn test_number_to_f64() {
        assert_eq!(f64::from(Number::U8(255)), 255.0);
        assert_eq!(f64::from(Number::I8(-1)), -1.0);
        assert_eq!(f64::from(Number::U16(65535)), 65535.0);
        assert_eq!(f64::from(Number::I16(-1)), -1.0);
        assert_eq!(f64::from(Number::U32(4294967295)), 4294967295.0);
        assert_eq!(f64::from(Number::I32(-1)), -1.0);
        assert_eq!(f64::from(Number::U64(18446744073709551615)), 18446744073709551615.0);
        assert_eq!(f64::from(Number::I64(-1)), -1.0);
    }

    #[test]
    fn test_number_to_u32() {
        assert_eq!(u32::from(Number::U8(255)), 255);
        assert_eq!(u32::from(Number::I8(-1)), u32::MAX);
        assert_eq!(u32::from(Number::U16(65535)), 65535);
        assert_eq!(u32::from(Number::I16(-1)), u32::MAX);
        assert_eq!(u32::from(Number::U32(4294967295)), 4294967295);
        assert_eq!(u32::from(Number::I32(-1)), u32::MAX);
    }

    #[test]
    fn test_number_to_i32() {
        assert_eq!(i32::from(Number::U8(255)), 255);
        assert_eq!(i32::from(Number::I8(-1)), -1);
        assert_eq!(i32::from(Number::U16(65535)), 65535);
        assert_eq!(i32::from(Number::I16(-1)), -1);
        assert_eq!(i32::from(Number::U32(4294967295)), 4294967295u32 as i32);
        assert_eq!(i32::from(Number::I32(-1)), -1);
    }

    #[test]
    fn test_number_to_f32() {
        assert_eq!(f32::from(Number::U8(255)), 255.0);
        assert_eq!(f32::from(Number::I8(-1)), -1.0);
        assert_eq!(f32::from(Number::U16(65535)), 65535.0);
        assert_eq!(f32::from(Number::I16(-1)), -1.0);
        assert_eq!(f32::from(Number::U32(4294967295)), 4294967295.0);
        assert_eq!(f32::from(Number::I32(-1)), -1.0);
    }

    #[test]
    fn test_number_to_u16() {
        assert_eq!(u16::from(Number::U8(255)), 255);
        assert_eq!(u16::from(Number::I8(-1)), u16::MAX);
        assert_eq!(u16::from(Number::U16(65535)), 65535);
        assert_eq!(u16::from(Number::I16(-1)), u16::MAX);
    }

    #[test]
    fn test_number_to_i16() {
        assert_eq!(i16::from(Number::U8(255)), 255);
        assert_eq!(i16::from(Number::I8(-1)), -1);
        assert_eq!(i16::from(Number::U16(65535)), 65535u16 as i16);
        assert_eq!(i16::from(Number::I16(-1)), -1);
    }

    #[test]
    fn test_number_to_u8() {
        assert_eq!(u8::from(Number::U8(255)), 255);
        assert_eq!(u8::from(Number::I8(-1)), u8::MAX);
    }

    #[test]
    fn test_number_to_i8() {
        assert_eq!(i8::from(Number::U8(255)), -1);
        assert_eq!(i8::from(Number::I8(-1)), -1);
    }

    #[test]
    fn test_number_to_attribute_value() {
        assert_eq!(
            AttributeValue::from(Number::U8(255)),
            AttributeValue::N("255".to_string())
        );
        assert_eq!(
            AttributeValue::from(Number::I8(-1)),
            AttributeValue::N("-1".to_string())
        );
        assert_eq!(
            AttributeValue::from(Number::F64(3.14)),
            AttributeValue::N("3.14".to_string())
        );
    }

    #[test]
    fn test_attribute_value_method() {
        let numbers = vec![Number::U8(1), Number::U8(2), Number::U8(3)];
        let attribute_value = Number::attribute_value(&numbers);
        if let AttributeValue::B(blob) = attribute_value {
            assert_eq!(blob.as_ref(), &[1, 2, 3]);
        } else {
            panic!("Expected AttributeValue::B");
        }

        let numbers = vec![Number::I32(1), Number::I32(2), Number::I32(3)];
        let attribute_value = Number::attribute_value(&numbers);
        if let AttributeValue::Ns(number_set) = attribute_value {
            assert_eq!(number_set, vec!["1", "2", "3"]);
        } else {
            panic!("Expected AttributeValue::Ns");
        }
    }
}