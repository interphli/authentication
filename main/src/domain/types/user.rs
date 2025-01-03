use aws_sdk_dynamodb::types::AttributeValue;
use serde::{Serialize, Deserialize, Serializer, Deserializer, ser::SerializeStruct};
use serde::de::{self, Visitor, MapAccess};
use std::fmt;
use std::error::Error as StdError;
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use lettre::Address;
use super::Id;

#[derive(Debug, Clone, PartialEq)]
pub enum EmailAddress {
    New(Address),
    Verified(Address)
}


#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Id,
    pub email: EmailAddress,
    pub user_name: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub profile_picture: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires: Option<DateTime<Utc>>
}


impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("User", 8)?;
        state.serialize_field("id", &self.id)?;
        match &self.email {
            EmailAddress::New(address) => {
                state.serialize_field("email", &address.to_string())?;
            }
            EmailAddress::Verified(address) => {
                state.serialize_field("email", &address.to_string())?;
                state.serialize_field("email_verified", &true)?;
            }
        }
        state.serialize_field("user_name", &self.user_name)?;
        state.serialize_field("first_name", &self.first_name)?;
        state.serialize_field("last_name", &self.last_name)?;
        state.serialize_field("password", &self.password)?;
        state.serialize_field("profile_picture", &self.profile_picture)?;
        state.serialize_field("created_at", &self.created_at)?;
        state.serialize_field("expires", &self.expires)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field { Id, Email, EmailVerified, UserName, FirstName, LastName, Password, ProfilePicture, CreatedAt, Expires }

        struct UserVisitor;

        impl<'de> Visitor<'de> for UserVisitor {
            type Value = User;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct User")
            }

            fn visit_map<V>(self, mut map: V) -> Result<User, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut id = None;
                let mut email = Option::<String>::None;
                let mut email_verified = None;
                let mut user_name = None;
                let mut first_name = None;
                let mut last_name = None;
                let mut password = None;
                let mut profile_picture = None;
                let mut created_at = None;
                let mut expires = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Id => {
                            if id.is_some() {
                                return Err(de::Error::duplicate_field("id"));
                            }
                            id = Some(map.next_value()?);
                        }
                        Field::Email => {
                            if email.is_some() {
                                return Err(de::Error::duplicate_field("email"));
                            }
                            email = Some(map.next_value()?);
                        }
                        Field::EmailVerified => {
                            if email_verified.is_some() {
                                return Err(de::Error::duplicate_field("email_verified"));
                            }
                            email_verified = Some(map.next_value()?);
                        }
                        Field::UserName => {
                            if user_name.is_some() {
                                return Err(de::Error::duplicate_field("user_name"));
                            }
                            user_name = Some(map.next_value()?);
                        }
                        Field::FirstName => {
                            if first_name.is_some() {
                                return Err(de::Error::duplicate_field("first_name"));
                            }
                            first_name = Some(map.next_value()?);
                        }
                        Field::LastName => {
                            if last_name.is_some() {
                                return Err(de::Error::duplicate_field("last_name"));
                            }
                            last_name = Some(map.next_value()?);
                        }
                        Field::Password => {
                            if password.is_some() {
                                return Err(de::Error::duplicate_field("password"));
                            }
                            password = Some(map.next_value()?);
                        }
                        Field::ProfilePicture => {
                            if profile_picture.is_some() {
                                return Err(de::Error::duplicate_field("profile_picture"));
                            }
                            profile_picture = Some(map.next_value()?);
                        }
                        Field::CreatedAt => {
                            if created_at.is_some() {
                                return Err(de::Error::duplicate_field("created_at"));
                            }
                            created_at = Some(map.next_value()?);
                        }
                        Field::Expires => {
                            if expires.is_some() {
                                return Err(de::Error::duplicate_field("expires"));
                            }
                            expires = Some(map.next_value()?);
                        }
                    }
                }
                let id = id.unwrap_or_default();
                let email = email.ok_or_else(|| de::Error::missing_field("email"))?;
                let email_verified = email_verified.unwrap_or(false);
                let address = email.parse().map_err(de::Error::custom)?;
                let email = if email_verified {
                    EmailAddress::Verified(address)
                } else {
                    EmailAddress::New(address)
                };
                let user_name = user_name.ok_or_else(|| de::Error::missing_field("user_name"))?;
                let first_name = first_name.ok_or_else(|| de::Error::missing_field("first_name"))?;
                let last_name = last_name.ok_or_else(|| de::Error::missing_field("last_name"))?;
                let password = password.ok_or_else(|| de::Error::missing_field("password"))?;
                let created_at = created_at.unwrap_or_else(Utc::now);
                let profile_picture = profile_picture.unwrap_or_default();
                let expires = expires.unwrap_or_default();
                
                Ok(User {
                    id,
                    email,
                    user_name,
                    first_name,
                    last_name,
                    password,
                    profile_picture,
                    created_at,
                    expires,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["id", "email", "email_verified", "user_name", "first_name", "last_name", "password", "profile_picture", "created_at", "expires"];
        deserializer.deserialize_struct("User", FIELDS, UserVisitor)
    }
}

impl TryFrom<AttributeValue> for User {
    type Error = Box<dyn StdError>;
    fn try_from(value: AttributeValue) -> Result<Self, Self::Error> {
        match value {
            AttributeValue::M(map) => map.try_into(),
            _ => Err("invalid Type")?
        }
    }
}


impl TryFrom<HashMap<String, AttributeValue>> for User {
    type Error = Box<dyn StdError>;
    fn try_from(mut map: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        let id = Id::try_from(map.remove("id").ok_or("field id not found")?)?;
        let email_verified = map.remove("email_verified").unwrap_or(AttributeValue::Bool(false));
        let email_verified = email_verified.as_bool().or(Err("expected a bool for the email_verified field."))?;
        let email = match map.remove("email").ok_or("could not get the email field")?{
            AttributeValue::S(address) => match *email_verified {
                true => Ok(EmailAddress::Verified(address.parse()?)),
                false => Ok(EmailAddress::New(address.parse()?))
            },
            _ => Err("expected a string for the field email")
        }?;
        let user_name = match map.remove("user_name").unwrap_or(AttributeValue::S(String::new())) {
            AttributeValue::S(value) => value,
            _ => Default::default()
        };
        let first_name = match map.remove("first_name").unwrap_or(AttributeValue::S(String::new())) {
            AttributeValue::S(value) => value,
            _ => Default::default()
        };
        let last_name = match map.remove("last_name").unwrap_or(AttributeValue::S(String::new())) {
            AttributeValue::S(value) => value,
            _ => Default::default()
        };
        let password = match map.remove("password").unwrap_or(AttributeValue::S(String::new())) {
            AttributeValue::S(value) => value,
            _ => Default::default()
        };
        let profile_picture = match map.remove("profile_picture") {
            None => None,
            Some(value) => {
                match value {
                    AttributeValue::S(value) => Some(value),
                    _ => Default::default()
                }
            }
        };
        let created_at = match map.remove("created_at").ok_or("the field created_at not found")?{
            AttributeValue::N(value) => {
                let milliseconds = value.parse()?;
                Ok(DateTime::from_timestamp_millis(milliseconds).ok_or("could not convert milliseconbds into a valid time")?)
            },
            _ => Err("expected a number for the field created_at")
        }?;
        let expires = match map.remove("expires") {
            None => Ok(None),
            Some(value) => {
                match value {
                    AttributeValue::N(value) => {
                        let milliseconds = value.parse()?;
                        Ok(Some(DateTime::from_timestamp_millis(milliseconds).ok_or("could not convert milliseconbds into a valid time")?))
                    },
                    _ => Err("expected a number for the field created_at")
                }
            }
        }?;
        Ok(User{id, email, user_name, first_name, last_name, password, profile_picture, created_at, expires})
    }
}


impl From<User> for AttributeValue {
    fn from(user: User) -> Self {
        AttributeValue::M(user.into())
    }
}


impl From<User> for HashMap<String, AttributeValue> {
    fn from(user: User) -> Self {
        let mut map = HashMap::new();
        map.insert("id".into(), user.id.into());
        match user.email {
            EmailAddress::New(address) => {map.insert("email".into(), AttributeValue::S(address.to_string()));},
            EmailAddress::Verified(address) => {
                map.insert("email_verified".into(), AttributeValue::Bool(true));
                map.insert("email".into(), AttributeValue::S(address.to_string()));
            }
        }
        map.insert("user_name".into(), AttributeValue::S(user.user_name));
        map.insert("first_name".into(), AttributeValue::S(user.first_name));
        map.insert("last_name".into(), AttributeValue::S(user.last_name));
        map.insert("password".into(), AttributeValue::S(user.password));
        map.insert("created_at".into(), AttributeValue::N(user.created_at.timestamp_millis().to_string()));
        if let Some(profile_picture) = user.profile_picture {map.insert("profile_picture".into(), AttributeValue::S(profile_picture));};
        if let Some(expires) = user.expires {map.insert("expires".into(), AttributeValue::N(expires.timestamp_millis().to_string()));};
        map
    }
}


impl From<EmailAddress> for AttributeValue {
    fn from(email: EmailAddress) -> Self {
        match email {
            EmailAddress::New(address) => AttributeValue::S(address.to_string()),
            EmailAddress::Verified(address) => AttributeValue::S(address.to_string())
        }
    }
}



#[cfg(test)]
    mod tests {
        use super::*;
        use serde_json;
        use aws_sdk_dynamodb::types::AttributeValue;
        use std::collections::HashMap;

        #[test]
        fn test_serialization() {
            let user = User {
                id: Id::default(),
                email: EmailAddress::New("test@example.com".parse().unwrap()),
                user_name: "testuser".to_string(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                password: "password".to_string(),
                profile_picture: Some("http://example.com/pic.jpg".to_string()),
                created_at: Utc::now(),
                expires: None,
            };

            let serialized = serde_json::to_string(&user).unwrap();
            assert!(serialized.contains("\"email\":\"test@example.com\""));
        }

        #[test]
        fn test_deserialization() {
            let data = r#"{
                "id": "000000000000000000000000",
                "email": "test@example.com",
                "user_name": "testuser",
                "first_name": "Test",
                "last_name": "User",
                "password": "password",
                "profile_picture": "http://example.com/pic.jpg",
                "created_at": "2024-12-26T00:00:00Z",
                "expires": null
            }"#;

            let user: User = serde_json::from_str(data).unwrap();
            assert_eq!(user.email, EmailAddress::New("test@example.com".parse().unwrap()));
        }

        #[test]
        fn test_try_from_attribute_value() {
            let mut map = HashMap::new();
            map.insert("id".to_string(), Id::default().into());
            map.insert("email".to_string(), AttributeValue::S("test@example.com".to_string()));
            map.insert("email_verified".to_string(), AttributeValue::Bool(false));
            map.insert("user_name".to_string(), AttributeValue::S("testuser".to_string()));
            map.insert("first_name".to_string(), AttributeValue::S("Test".to_string()));
            map.insert("last_name".to_string(), AttributeValue::S("User".to_string()));
            map.insert("password".to_string(), AttributeValue::S("password".to_string()));
            map.insert("created_at".to_string(), AttributeValue::N(Utc::now().timestamp_millis().to_string()));

            let user: User = map.try_into().unwrap();
            assert_eq!(user.email, EmailAddress::New("test@example.com".parse().unwrap()));
        }

        #[test]
        fn test_from_user_to_attribute_value() {
            let user = User {
                id: Id::default(),
                email: EmailAddress::New("test@example.com".parse().unwrap()),
                user_name: "testuser".to_string(),
                first_name: "Test".to_string(),
                last_name: "User".to_string(),
                password: "password".to_string(),
                profile_picture: Some("http://example.com/pic.jpg".to_string()),
                created_at: Utc::now(),
                expires: None,
            };

            let attribute_value: AttributeValue = user.into();
            if let AttributeValue::M(map) = attribute_value {
                assert!(map.contains_key("email"));
            } else {
                panic!("Expected AttributeValue::M");
            }
        }
    }