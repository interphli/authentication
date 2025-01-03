use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error as StdError;
use super::Number;


#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(untagged)]
pub enum Value {
    #[default]
    None,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>)
}


impl Value {
    pub fn as_option<T: TryFrom<Value, Error = Box<dyn StdError>>>(self) -> Option<T> {
        match self {
            Value::None => None,
            _ => T::try_from(self).ok()
        }
    }

    pub fn option<T: TryFrom<Value, Error = Box<dyn StdError>>>(option: Option<Value>) -> Option<T> {
        match option {
            None => None,
            Some(value) => {
                match value {
                    Value::None => None,
                    _ => T::try_from(value).ok()
                }
            }
        }
    }

    pub fn from_option<T: TryFrom<Value, Error = Box<dyn StdError>>>(option: Option<Value>) -> Result<T, Box<dyn StdError>> {
        match option {
            None => Err("None value cannot be converted".into()),
            Some(value) => T::try_from(value)
        }
    }

    pub fn as_int<T: TryFrom<Number, Error = Box<dyn StdError>> + Default>(&self) -> T {
        match self {
            Value::Number(number) => T::try_from(*number).unwrap_or_default(),
            _ => Default::default()
        }
    }
}


impl TryFrom<Value> for bool {
    type Error = Box<dyn StdError>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(boolean) => Ok(boolean),
            _ => Err("Cannot convert to bool".into())
        }
    }
}

impl<T: TryFrom<Number, Error = Box<dyn StdError>> + Default> TryFrom<Value> for (T,) {
    type Error = Box<dyn StdError>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(number) => Ok((T::try_from(number)?,)),
            _ => Err("Cannot convert to tuple".into())
        }
    }
}

impl TryFrom<Value> for String {
    type Error = Box<dyn StdError>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::String(string) => Ok(string),
            _ => Err("Cannot convert to String".into())
        }
    }
}


impl<T: TryFrom<Value, Error = Box<dyn StdError>>> TryFrom<Value> for HashMap<String, T> {
    type Error = Box<dyn StdError>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Map(map) => map.into_iter().map(|(key, value)| {
                Ok((key, T::try_from(value)?))
            }).collect(),
            _ => Err("Cannot convert to HashMap".into())
        }
    }
}


impl<T: TryFrom<Value, Error = Box<dyn StdError>>> TryFrom<Value> for Vec<T> {
    type Error = Box<dyn StdError>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Array(array) => array.into_iter().map(T::try_from).collect(),
            _ => Err("Cannot convert to Vec".into())
        }
    }
}


impl TryFrom<Value> for HashMap<String, Value> {
    type Error = Box<dyn StdError>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Map(map) = value {
            Ok(map)
        } else {
            Err("Cannot convert to HashMap".into())
        }
    }
}

impl TryFrom<Value> for Vec<Value> {
    type Error = Box<dyn StdError>;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Value::Array(array) = value {
            Ok(array)
        } else {
            Err("Cannot convert to Vec".into())
        }
    }
}


impl From<Value> for AttributeValue {
    fn from(value: Value) -> Self {
        match value {
            Value::None => AttributeValue::Null(true),
            Value::Bool(bool) => AttributeValue::Bool(bool),
            Value::Number(number) => number.try_into().unwrap_or_else(|_| AttributeValue::Null(true)),
            Value::String(string) => AttributeValue::S(string),
            Value::Map(map) => {
                let converted_map: HashMap<String, AttributeValue> = map.into_iter()
                    .map(|(key, value)| (key, AttributeValue::from(value)))
                    .collect();
                AttributeValue::M(converted_map)
            }
            Value::Array(values) => {
                if values.is_empty() {
                    AttributeValue::L(vec![])
                } else if matches!(values[0], Value::Number(_)) {
                    if values.iter().all(|v| matches!(v, Value::Number(_))) {
                        let numbers: Vec<Number> = values.into_iter().filter_map(|v| {
                            if let Value::Number(num) = v {
                                Some(num)
                            } else {
                                None
                            }
                        }).collect();
                        Number::attribute_value(&numbers)
                    } else {
                        let list: Vec<AttributeValue> = values.into_iter().map(AttributeValue::from).collect();
                        AttributeValue::L(list)
                    }
                } else if matches!(values[0], Value::String(_)) {
                    if values.iter().all(|v| matches!(v, Value::String(_))) {
                        let ss: Vec<String> = values.into_iter().map(|v| {
                            if let Value::String(s) = v {
                                s
                            } else {
                                String::new()
                            }
                        }).collect();
                        AttributeValue::Ss(ss)
                    } else {
                        let list: Vec<AttributeValue> = values.into_iter().map(AttributeValue::from).collect();
                        AttributeValue::L(list)
                    }
                } else {
                    let list: Vec<AttributeValue> = values.into_iter().map(AttributeValue::from).collect();
                    AttributeValue::L(list)
                }
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_value_as_option() {
        assert_eq!(Value::None.as_option::<bool>(), None);
        assert_eq!(Value::Bool(true).as_option::<bool>(), Some(true));
    }

    #[test]
    fn test_value_option() {
        assert_eq!(Value::option::<bool>(None), None);
        assert_eq!(Value::option::<bool>(Some(Value::None)), None);
        assert_eq!(Value::option(Some(Value::Bool(true))), Some(true));
    }

    #[test]
    fn test_value_from_option() {
        assert!(Value::from_option::<bool>(None).is_err());
        assert_eq!(Value::from_option::<bool>(Some(Value::Bool(true))).unwrap(), true);
    }

    #[test]
    fn test_value_as_int() {
        assert_eq!(Value::Number(Number::U8(42)).as_int::<u8>(), 42);
        assert_eq!(Value::String("not a number".to_string()).as_int::<u8>(), 0);
    }

    #[test]
    fn test_value_to_option() {
        assert_eq!(<(u8,)>::try_from(Value::Number(Number::U8(42))).unwrap(), (42u8,));
        assert!(<(u8,)>::try_from(Value::String("not a number".to_string())).is_err());
    }

    #[test]
    fn test_value_to_string() {
        assert_eq!(String::try_from(Value::String("hello".to_string())).unwrap(), "hello".to_string());
        assert!(String::try_from(Value::Bool(true)).is_err());
    }

    #[test]
    fn test_value_to_hashmap() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Bool(true));
        assert_eq!(HashMap::<String, Value>::try_from(Value::Map(map.clone())).unwrap(), map);
        assert!(HashMap::<String, Value>::try_from(Value::Bool(true)).is_err());
    }

    #[test]
    fn test_value_to_vec() {
        let vec = vec![Value::Bool(true), Value::Bool(false)];
        assert_eq!(Vec::<Value>::try_from(Value::Array(vec.clone())).unwrap(), vec);
        assert!(Vec::<Value>::try_from(Value::Bool(true)).is_err());
    }

    #[test]
    fn test_value_to_attribute_value() {
        assert_eq!(AttributeValue::from(Value::None), AttributeValue::Null(true));
        assert_eq!(AttributeValue::from(Value::Bool(true)), AttributeValue::Bool(true));
        assert_eq!(AttributeValue::from(Value::String("test".to_string())), AttributeValue::S("test".to_string()));

        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Bool(true));
        assert_eq!(
            AttributeValue::from(Value::Map(map.clone())),
            AttributeValue::M(map.into_iter().map(|(k, v)| (k, AttributeValue::try_from(v).unwrap())).collect())
        );

        let vec = vec![Value::Number(Number::U8(1)), Value::Number(Number::U8(2))];
        assert_eq!(
            AttributeValue::from(Value::Array(vec.clone())),
            Number::attribute_value(&vec.into_iter().filter_map(|v| if let Value::Number(n) = v { Some(n) } else { None }).collect())
        );
    }
}
