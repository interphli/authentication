use aws_sdk_dynamodb::types::AttributeValue;
use std::error::Error as StdError;
use std::collections::HashMap;
use chrono::{DateTime, Utc, TimeZone};
use std::convert::TryFrom;
use std::str::FromStr;
use uuid::Uuid;
use super::Id;

#[derive(Debug, Clone, PartialEq)]
pub struct Verification {
    pub user_id: Id,
    pub magic_id: Uuid,
    pub code: u32,
    pub expires: DateTime<Utc>
}


impl From<Verification> for HashMap<String, AttributeValue> {
    fn from(verification: Verification) -> Self {
        let mut map = HashMap::new();
        map.insert("user_id".to_string(), AttributeValue::B(verification.user_id.bytes().as_slice().into()));
        map.insert("magic_id".to_string(), AttributeValue::B(verification.magic_id.as_bytes().as_slice().into()));
        map.insert("code".to_string(), AttributeValue::N(verification.code.to_string()));
        map.insert("expires".to_string(), AttributeValue::N(verification.expires.timestamp().to_string()));
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
            Some(AttributeValue::N(s)) => s.parse()?,
            _ => return Err("code not found or invalid".into()),
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
            code: code,
            expires,
        })
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use aws_sdk_dynamodb::types::AttributeValue;
    use chrono::{TimeZone, Utc};
    use uuid::Uuid;

    #[test]
    fn test_verification_to_hashmap() {
        let verification = Verification {
            user_id: Id::from_str("507f1f77bcf86cd799439011").unwrap(),
            magic_id: Uuid::new_v4(),
            code: 120203,
            expires: DateTime::from_timestamp(1_614_000_600, 0).unwrap(),
        };

        let map: HashMap<String, AttributeValue> = verification.clone().into();
        assert_eq!(map.get("code").unwrap().as_n().unwrap(), "120203");
        assert_eq!(map.get("expires").unwrap().as_n().unwrap(), "1614000600");
    }

    #[test]
    fn test_hashmap_to_verification() {
        let mut map = HashMap::new();
        map.insert("user_id".to_string(), AttributeValue::B(b"507f1f77bcf86cd799439011".to_vec().into()));
        map.insert("magic_id".to_string(), AttributeValue::B(Uuid::new_v4().as_bytes().to_vec().into()));
        map.insert("code".to_string(), AttributeValue::N(010203.to_string()));
        map.insert("expires".to_string(), AttributeValue::S("1614000600".to_string()));

        let verification = Verification::try_from(map).unwrap();
        assert_eq!(verification.code, 010203);
        assert_eq!(verification.expires.timestamp(), 1_614_000_600);
    }
}
