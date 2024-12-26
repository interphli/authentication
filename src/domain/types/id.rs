use serde::{Serialize, Deserialize, Serializer};
use aws_sdk_dynamodb::types::AttributeValue;
use std::ops::{Deref, DerefMut};
use bson::oid::ObjectId;
use std::str::FromStr;
use crate::{Result, StdError};


#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
pub struct Id(ObjectId);



impl Deref for Id {
    type Target = ObjectId;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Id {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl FromStr for Id {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Id(ObjectId::from_str(s)?))
    }
}


impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str(&self.0.to_hex())
    }
}


impl From<Id> for AttributeValue {
    fn from(id: Id) -> Self {
        let bytes = id.bytes();
        AttributeValue::B(bytes.as_ref().into())
    }
}

impl TryFrom<AttributeValue> for Id {
    type Error = Box<dyn StdError>;
    fn try_from(value: AttributeValue) -> Result<Self> {
        match value {
            AttributeValue::B(blob) => {
                let bytes = blob.into_inner();
                let array: [u8; 12] = bytes.as_slice().try_into().map_err(|_|"invalid id length")?;
                let oid = array.into();
                Ok(Id(oid))
            },
            AttributeValue::S(string) => {
                let bytes = Vec::from(string);
                let array: [u8; 12] = bytes.as_slice().try_into().map_err(|_|"invalid id length")?;
                let oid = array.into();
                Ok(Id(oid))
            },
            _ => Err("invalid id type")?
        }
    }
}