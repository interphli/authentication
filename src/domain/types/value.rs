use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
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
    pub fn as_option<T: From<Value>>(self) -> Option<T> {
        match self {
            Value::None => None,
            _ => Some(self.into())
        }
    }

    pub fn option<T: From<Value>>(option: Option<Value>) -> Option<T> {
        match option {
            None => None,
            Some(value) => {
                match value {
                    Value::None => None,
                    _ => Some(value.into())
                }
            }
        }
    }

    pub fn from_option<T: From<Value>>(option: Option<Value>) -> T {
        match option {
            None => Value::None.into(),
            Some(value) => value.into()
        }
    }

    pub fn as_int<T: From<Number> + Default>(&self) -> T
    {
        match self {
            Value::Number(number) => T::from(*number),
            _ => Default::default()
        }
    }
}


impl From<Value> for bool {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(boolean) => boolean,
            _ => Default::default()
        }
    }
}


impl<T: From<Number> + Default> From<Value> for Option<T> {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(number) => Some(number.into()),
            _ => Default::default()
        }
    }
}

impl<T: From<Number> + Default> From<Value> for (T,) {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(number) => (number.into(), ),
            _ => Default::default()
        }
    }
}

impl From<Value> for String {
    fn from(value: Value) -> Self {
        match value {
            Value::String(string) => string,
            _ => Default::default()
        }
    }
}


impl<T: From<Value>> From<Value> for HashMap<String, T> {
    fn from(value: Value) -> Self {
        match value {
            Value::Map(map) => map.into_iter().map(|(key, value)|{(key, T::from(value))}).collect(),
            _ => Default::default()
        }
    }
}


impl<T: From<Value>> From<Value> for Vec<T> {
    fn from(value: Value) -> Self {
        match value {
            Value::Array(array) => array.into_iter().map(|value|{T::from(value)}).collect(),
            _ => Default::default()
        }
    }
}


impl From<Value> for AttributeValue {
    fn from(value: Value) -> Self {
        match value {
            Value::None => AttributeValue::Null(true),
            Value::Bool(bool) => AttributeValue::Bool(bool),
            Value::Number(number) => number.into(),
            Value::String(string) => AttributeValue::S(string),
            Value::Map(map) => AttributeValue::M(map.into_iter().map(|(key, value)|{(key, value.into())}).collect()),
            Value::Array(values) => {
                if values.is_empty() {
                    AttributeValue::L(vec![])
                } else if matches!(values[0], Value::Number(_)) {
                    // Check if all elements are numbers
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
                        // Default to L
                        let list: Vec<AttributeValue> = values.into_iter().map(AttributeValue::from).collect();
                        AttributeValue::L(list)
                    }
                } else if matches!(values[0], Value::String(_)) {
                    // Check if all elements are strings
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
                        // Default to L
                        let list: Vec<AttributeValue> = values.into_iter().map(AttributeValue::from).collect();
                        AttributeValue::L(list)
                    }
                } else {
                    // Default to L
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
    use aws_sdk_dynamodb::types::AttributeValue;

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
        assert_eq!(Value::from_option::<bool>(None), false);
        assert_eq!(Value::from_option::<bool>(Some(Value::Bool(true))), true);
    }

    #[test]
    fn test_value_as_int() {
        assert_eq!(Value::Number(Number::U8(42)).as_int::<u8>(), 42);
        assert_eq!(Value::String("not a number".to_string()).as_int::<u8>(), 0);
    }

    #[test]
    fn test_value_to_option() {
        assert_eq!(Option::<u8>::from(Value::Number(Number::U8(42))), Some(42u8));
        assert_eq!(Option::<u8>::from(Value::String("not a number".to_string())), None);
    }

    #[test]
    fn test_value_to_string() {
        assert_eq!(String::from(Value::String("hello".to_string())), "hello".to_string());
        assert_eq!(String::from(Value::Bool(true)), "".to_string());
    }

    #[test]
    fn test_value_to_hashmap() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), Value::Bool(true));
        assert_eq!(HashMap::<String, Value>::from(Value::Map(map.clone())), map);
        assert_eq!(HashMap::<String, Value>::from(Value::Bool(true)), HashMap::new());
    }

    #[test]
    fn test_value_to_vec() {
        let vec = vec![Value::Bool(true), Value::Bool(false)];
        assert_eq!(Vec::<Value>::from(Value::Array(vec.clone())), vec);
        assert_eq!(Vec::<Value>::from(Value::Bool(true)), Vec::new());
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
            AttributeValue::M(map.into_iter().map(|(k, v)| (k, v.into())).collect())
        );

        let vec = vec![Value::Number(Number::U8(1)), Value::Number(Number::U8(2))];
        assert_eq!(
            AttributeValue::from(Value::Array(vec.clone())),
            Number::attribute_value(&vec.into_iter().filter_map(|v| if let Value::Number(n) = v { Some(n) } else { None }).collect())
        );
    }
}
