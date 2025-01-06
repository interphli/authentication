use aws_sdk_ssm::types::ParameterType;
use serde::{Serialize, Deserialize};
use std::error::Error as StdError;
use chrono::{Utc, DateTime};
use aws_sdk_ssm::Client;


pub use aws_sdk_ssm;


pub const PARAMETER_NAME: &'static str = "PASETO_KEY";


type Result<T> = std::result::Result<T, Box<dyn StdError + Send + Sync>>;


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Keys {
    pub private_key: [u8; 32],
    pub public_key: [u8; 32],
    pub prev_public_key: Option<[u8; 32]>,
    pub created_time: DateTime<Utc>,
    pub expires: DateTime<Utc>
}



impl Keys {
    pub async fn get(client: &Client) -> Result<Option<Keys>> {
        let name = PARAMETER_NAME;
        let output = client.get_parameter().name(name).with_decryption(true).send().await?;
        let parameter = match output.parameter {
            Some(parameter) => parameter,
            None => return Ok(None)
        };
        let json = match parameter.value {
            Some(json) => json,
            None => return Ok(None)
        };
        let keys = serde_json::from_str(&json)?;
        Ok(Some(keys))
    }
    
    pub async fn put(&self, client: &Client) -> Result<()> {
        let json = serde_json::to_string(self)?;
        let name = PARAMETER_NAME;
        let r#type = ParameterType::SecureString;
        let _ = client.put_parameter().name(name).r#type(r#type).value(json).send().await?;
        Ok(())
    }
}