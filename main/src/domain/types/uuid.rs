use serde::{Serialize, Deserialize, Serializer};
use aws_sdk_dynamodb::types::AttributeValue;
use std::ops::{Deref, DerefMut};
use uuid::Uuid as Uid;
use std::str::FromStr;
use crate::{Result, StdError};


#[derive(Clone, Debug, Deserialize, Default, PartialEq)]
pub struct Uuid(Uid);


impl Uuid {
    pub fn new_v4() -> Self {
        Self(Uid::new_v4())
    }
}


impl Deref for Uuid {
    type Target = Uid;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Uuid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl FromStr for Uuid {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(Uuid(Uid::from_str(s)?))
    }
}


impl Serialize for Uuid {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
        where
            S: Serializer {
        serializer.serialize_str(&self.0.simple().to_string())
    }
}


impl From<Uuid> for AttributeValue {
    fn from(id: Uuid) -> Self {
        let bytes = id.0.as_bytes();
        AttributeValue::B(bytes.as_ref().into())
    }
}

impl TryFrom<AttributeValue> for Uuid {
    type Error = Box<dyn StdError>;
    fn try_from(value: AttributeValue) -> Result<Self> {
        match value {
            AttributeValue::B(blob) => {
                let bytes = blob.into_inner();
                let uuid = Uid::from_slice(bytes.as_slice()).map_err(|_|"invalid uuid length")?;
                Ok(Uuid(uuid))
            },
            AttributeValue::S(string) => {
                let uuid = string.parse()?;
                Ok(Uuid(uuid))
            },
            _ => Err("invalid id type")?
        }
    }
}