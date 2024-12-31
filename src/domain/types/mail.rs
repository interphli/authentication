use lettre::{message::Mailbox, transport::smtp::PoolConfig, AsyncSmtpTransport, Tokio1Executor};
use lettre::transport::smtp::authentication::Credentials;
use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, MapAccess, Visitor};
use std::fmt;
use super::StdError;


pub type Mailer = AsyncSmtpTransport<Tokio1Executor>;
type Result<T> = std::result::Result<T, StdError>;


#[derive(Debug, Clone, Serialize)]
pub struct Mail {
    credentials: Option<Credentials>,
    url: String,
    sender: Mailbox,
    #[serde(skip)] // Skip during serialization
    mailer: Mailer
}


impl Mail {
    fn init(url: &str, credentials: &Option<Credentials>) -> Result<Mailer> {
        let connection_url = url;
        let mut mailer = Mailer::from_url(connection_url)?;
        if let Some(credentials) = credentials {
            mailer = mailer.credentials(credentials.clone())
        }
        Ok(mailer.pool_config(PoolConfig::new()).build())
    }

    pub fn mailer(&self) -> &Mailer {
        &self.mailer
    }

    pub fn credentials(&self) -> &Option<Credentials> {
        &self.credentials
    }

    pub fn sender(&self) -> &Mailbox {
        &self.sender
    }
}


impl<'de> Deserialize<'de> for Mail {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct MailData {
            credentials: Option<Credentials>,
            url: String,
            sender: Mailbox,
        }

        struct MailVisitor;

        impl<'de> Visitor<'de> for MailVisitor {
            type Value = Mail;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Mail")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Mail, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut credentials = None;
                let mut url = Option::<String>::None;
                let mut sender = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "credentials" => {
                            credentials = Some(map.next_value()?);
                        }
                        "url" => {
                            url = Some(map.next_value()?);
                        }
                        "sender" => {
                            sender = Some(map.next_value()?);
                        }
                        _ => {
                            let _: de::IgnoredAny = map.next_value()?;
                        }
                    }
                }

                let credentials = credentials.ok_or_else(|| de::Error::missing_field("credentials"))?;
                let url = url.ok_or_else(|| de::Error::missing_field("url"))?;
                let sender = sender.ok_or_else(|| de::Error::missing_field("sender"))?;

                // Initialize the mailer using the provided function
                let mailer = Mail::init(&url, &credentials).map_err(de::Error::custom)?;

                Ok(Mail {
                    credentials,
                    url,
                    sender,
                    mailer,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["credentials", "url", "sender"];
        deserializer.deserialize_struct("Mail", FIELDS, MailVisitor)
    }
}