use aws_sdk_dynamodb::types::AttributeValue;
use std::error::Error as StdError;
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;
use super::Id;

#[derive(Debug, Clone, PartialEq)]
pub struct Verification {
    pub user_id: Id,
    pub magic_id: Uuid,
    pub code: String,
    pub created_at: DateTime<Utc>,
    pub expires: DateTime<Utc>
}

impl From<Verification> for HashMap<String, AttributeValue> {
    fn from(verification: Verification) -> Self {
        let mut map = HashMap::new();
        map.insert("user_id".to_string(), AttributeValue::B(verification.user_id.bytes().as_slice().into()));
        map.insert("magic_id".to_string(), AttributeValue::B(verification.magic_id.as_bytes().as_slice().into()));
        map.insert("code".to_string(), AttributeValue::S(verification.code));
        map.insert("created_at".to_string(), AttributeValue::S(verification.created_at.timestamp().to_string()));
        map.insert("expires".to_string(), AttributeValue::S(verification.expires.timestamp().to_string()));
        map
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for Verification {
    type Error = Box<dyn StdError>;

    fn try_from(mut map: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let user_id = match map.remove("user_id") {
            Some(AttributeValue::B(bytes)) => bytes.into_inner(),
            _ => return Err("user_id not found or invalid".into()),
        };

        let magic_id = match map.remove("magic_id") {
            Some(AttributeValue::B(bytes)) => bytes.into_inner(),
            _ => return Err("magic_id not found or invalid".into()),
        };

        let code = match map.remove("code") {
            Some(AttributeValue::S(s)) => s,
            _ => return Err("code not found or invalid".into()),
        };

        let created_at = match map.remove("created_at") {
            Some(AttributeValue::S(s)) => {
                let secs = s.parse::<i64>().map_err(|_| "invalid timestamp for field created_at")?;
                DateTime::from_timestamp(secs, 0).ok_or("invalid timestamp for field created_at")?
            },
            _ => return Err("created_at not found or invalid".into()),
        };

        let expires = match map.remove("expires") {
            Some(AttributeValue::S(s)) => {
                let secs = s.parse::<i64>().map_err(|_| "invalid timestamp for field expires")?;
                DateTime::from_timestamp(secs, 0).ok_or("invalid timestamp for field expires")?
            },
            _ => return Err("expires not found or invalid".into()),
        };

        Ok(Verification {
            user_id: std::str::from_utf8(user_id.as_slice()).map_err(|_| "Invalid user_id")?.parse()?,
            magic_id: Uuid::from_slice(magic_id.as_slice()).map_err(|_| "Invalid magic_id")?,
            code: code.to_string(),
            created_at,
            expires,
        })
    }
}
