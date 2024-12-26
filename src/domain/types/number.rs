use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Serialize, Deserialize};
use std::fmt::{Formatter, Display};
use std::convert::TryFrom;
use std::error::Error as StdError;

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


macro_rules! try_from_number_float {
    ($target:ty, $variant:ident) => {
        impl TryFrom<Number> for $target {
            type Error = Box<dyn StdError>;

            fn try_from(number: Number) -> Result<Self, Self::Error> {
                Ok(match number {
                    Number::U8(value) => value as $target,
                    Number::I8(value) => value as $target,
                    Number::U16(value) => value as $target,
                    Number::I16(value) => value as $target,
                    Number::U32(value) => value as $target,
                    Number::I32(value) => value as $target,
                    Number::U64(value) => value as $target,
                    Number::I64(value) => value as $target,
                    Number::U128(value) => value as $target,
                    Number::I128(value) => value as $target,
                    Number::F32(value) => value as $target,
                    Number::F64(value) => value as $target,
                })
            }
        }

        impl From<$target> for Number {
            fn from(value: $target) -> Self {
                Number::$variant(value)
            }
        }
    };
}

macro_rules! try_from_number_general {
    ($target:ty, $variant:ident) => {
        impl TryFrom<Number> for $target {
            type Error = Box<dyn StdError>;

            fn try_from(number: Number) -> Result<Self, Self::Error> {
                match number {
                    Number::U8(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::I8(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::U16(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::I16(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::U32(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::I32(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::U64(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::I64(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::U128(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::I128(value) => value.try_into().map_err(|_| format!("Cannot convert {:?} to {}", number, stringify!($target)).into()),
                    Number::F32(value) => Ok(value as $target),
                    Number::F64(value) => Ok(value as $target),
                }
            }
        }

        impl From<$target> for Number {
            fn from(value: $target) -> Self {
                Number::$variant(value)
            }
        }
    };
}

// Use the appropriate macro for each type
try_from_number_float!(f32, F32);
try_from_number_float!(f64, F64);

try_from_number_general!(u8, U8);
try_from_number_general!(i8, I8);
try_from_number_general!(u16, U16);
try_from_number_general!(i16, I16);
try_from_number_general!(u32, U32);
try_from_number_general!(i32, I32);
try_from_number_general!(u64, U64);
try_from_number_general!(i64, I64);
try_from_number_general!(u128, U128);
try_from_number_general!(i128, I128);


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


impl TryFrom<Number> for AttributeValue {
    type Error = Box<dyn StdError>;

    fn try_from(number: Number) -> Result<Self, Self::Error> {
        let number = number.to_string();
        Ok(AttributeValue::N(number))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_number_to_u128() {
        assert_eq!(u128::try_from(Number::U8(255)).unwrap(), 255);
        assert!(u128::try_from(Number::I8(-1)).is_err());
        assert_eq!(u128::try_from(Number::U16(65535)).unwrap(), 65535);
        assert!(u128::try_from(Number::I16(-1)).is_err());
        assert_eq!(u128::try_from(Number::U32(4294967295)).unwrap(), 4294967295);
        assert!(u128::try_from(Number::I32(-1)).is_err());
        assert_eq!(u128::try_from(Number::U64(18446744073709551615)).unwrap(), 18446744073709551615);
        assert!(u128::try_from(Number::I64(-1)).is_err());
        assert_eq!(u128::try_from(Number::U128(340282366920938463463374607431768211455)).unwrap(), 340282366920938463463374607431768211455);
        assert!(u128::try_from(Number::I128(-1)).is_err());
    }

    #[test]
    fn test_number_to_i128() {
        assert_eq!(i128::try_from(Number::U8(255)).unwrap(), 255);
        assert_eq!(i128::try_from(Number::I8(-1)).unwrap(), -1);
        assert_eq!(i128::try_from(Number::U16(65535)).unwrap(), 65535);
        assert_eq!(i128::try_from(Number::I16(-1)).unwrap(), -1);
        assert_eq!(i128::try_from(Number::U32(4294967295)).unwrap(), 4294967295);
        assert_eq!(i128::try_from(Number::I32(-1)).unwrap(), -1);
        assert_eq!(i128::try_from(Number::U64(18446744073709551615)).unwrap(), 18446744073709551615);
        assert_eq!(i128::try_from(Number::I64(-1)).unwrap(), -1);
        assert!(i128::try_from(Number::U128(340282366920938463463374607431768211455)).is_err());
        assert_eq!(i128::try_from(Number::I128(-1)).unwrap(), -1);
    }

    #[test]
    fn test_number_to_u64() {
        assert_eq!(u64::try_from(Number::U8(255)).unwrap(), 255);
        assert!(u64::try_from(Number::I8(-1)).is_err());
        assert_eq!(u64::try_from(Number::U16(65535)).unwrap(), 65535);
        assert!(u64::try_from(Number::I16(-1)).is_err());
        assert_eq!(u64::try_from(Number::U32(4294967295)).unwrap(), 4294967295);
        assert!(u64::try_from(Number::I32(-1)).is_err());
        assert_eq!(u64::try_from(Number::U64(18446744073709551615)).unwrap(), 18446744073709551615);
        assert!(u64::try_from(Number::I64(-1)).is_err());
    }

    #[test]
    fn test_number_to_i64() {
        assert_eq!(i64::try_from(Number::U8(255)).unwrap(), 255);
        assert_eq!(i64::try_from(Number::I8(-1)).unwrap(), -1);
        assert_eq!(i64::try_from(Number::U16(65535)).unwrap(), 65535);
        assert_eq!(i64::try_from(Number::I16(-1)).unwrap(), -1);
        assert_eq!(i64::try_from(Number::U32(4294967295)).unwrap(), 4294967295);
        // assert_eq!(i64::try_from(Number::I32(-1)).unwrap(), -1);
        assert!(i64::try_from(Number::U64(18446744073709551615)).is_err());
        assert_eq!(i64::try_from(Number::I64(-1)).unwrap(), -1);
    }

    #[test]
    fn test_number_to_f64() {
        assert_eq!(f64::try_from(Number::U8(255)).unwrap(), 255.0);
        assert_eq!(f64::try_from(Number::I8(-1)).unwrap(), -1.0);
        assert_eq!(f64::try_from(Number::U16(65535)).unwrap(), 65535.0);
        assert_eq!(f64::try_from(Number::I16(-1)).unwrap(), -1.0);
        assert_eq!(f64::try_from(Number::U32(4294967295)).unwrap(), 4294967295.0);
        assert_eq!(f64::try_from(Number::I32(-1)).unwrap(), -1.0);
        assert_eq!(f64::try_from(Number::U64(18446744073709551615)).unwrap(), 18446744073709551615.0);
        assert_eq!(f64::try_from(Number::I64(-1)).unwrap(), -1.0);
    }

    #[test]
    fn test_number_to_u32() {
        assert_eq!(u32::try_from(Number::U8(255)).unwrap(), 255);
        assert!(u32::try_from(Number::I8(-1)).is_err());
        assert_eq!(u32::try_from(Number::U16(65535)).unwrap(), 65535);
        assert!(u32::try_from(Number::I16(-1)).is_err());
        assert_eq!(u32::try_from(Number::U32(4294967295)).unwrap(), 4294967295);
        assert!(u32::try_from(Number::I32(-1)).is_err());
    }

    #[test]
    fn test_number_to_i32() {
        assert_eq!(i32::try_from(Number::U8(255)).unwrap(), 255);
        assert_eq!(i32::try_from(Number::I8(-1)).unwrap(), -1);
        assert_eq!(i32::try_from(Number::U16(65535)).unwrap(), 65535);
        assert_eq!(i32::try_from(Number::I16(-1)).unwrap(), -1);
        assert!(i32::try_from(Number::U32(4294967295)).is_err());
        assert_eq!(i32::try_from(Number::I32(-1)).unwrap(), -1);
    }

    #[test]
    fn test_number_to_f32() {
        assert_eq!(f32::try_from(Number::U8(255)).unwrap(), 255.0);
        assert_eq!(f32::try_from(Number::I8(-1)).unwrap(), -1.0);
        assert_eq!(f32::try_from(Number::U16(65535)).unwrap(), 65535.0);
        assert_eq!(f32::try_from(Number::I16(-1)).unwrap(), -1.0);
        assert_eq!(f32::try_from(Number::U32(4294967295)).unwrap(), 4294967295.0);
        assert_eq!(f32::try_from(Number::I32(-1)).unwrap(), -1.0);
    }

    #[test]
    fn test_number_to_u16() {
        assert_eq!(u16::try_from(Number::U8(255)).unwrap(), 255);
        assert!(u16::try_from(Number::I8(-1)).is_err());
        assert_eq!(u16::try_from(Number::U16(65535)).unwrap(), 65535);
        assert!(u16::try_from(Number::I16(-1)).is_err());
    }

    #[test]
    fn test_number_to_i16() {
        assert_eq!(i16::try_from(Number::U8(255)).unwrap(), 255);
        assert_eq!(i16::try_from(Number::I8(-1)).unwrap(), -1);
        assert!(i16::try_from(Number::U16(65535)).is_err());
        assert_eq!(i16::try_from(Number::I16(-1)).unwrap(), -1);
    }

    #[test]
    fn test_number_to_u8() {
        assert_eq!(u8::try_from(Number::U8(255)).unwrap(), 255);
        assert!(u8::try_from(Number::I8(-1)).is_err());
    }

    #[test]
    fn test_number_to_i8() {
        assert!(i8::try_from(Number::U8(255)).is_err());
        assert_eq!(i8::try_from(Number::I8(-1)).unwrap(), -1);
    }

    #[test]
    fn test_number_to_attribute_value() {
        assert_eq!(
            AttributeValue::try_from(Number::U8(255)).unwrap(),
            AttributeValue::N("255".to_string())
        );
        assert_eq!(
            AttributeValue::try_from(Number::I8(-1)).unwrap(),
            AttributeValue::N("-1".to_string())
        );
        assert_eq!(
            AttributeValue::try_from(Number::F64(3.14)).unwrap(),
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
