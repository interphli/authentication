use aws_sdk_lambda::types::InvocationType;
use aws_sdk_lambda::primitives::Blob;
use serde_json::{to_vec as json, from_slice};
use shared::{Request, Result as Response};
use aws_sdk_lambda::Client;
use std::env::var;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
pub struct PasswordHasher {
    client: Client
}


impl PasswordHasher {
    pub async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self{client}
    }

    pub async fn hash(&self, password: String) -> Result<String> {
        let function_name = var("ARGON").unwrap_or(String::from("argon"));
        let request = Request::Hash(password);
        let json = json(&request)?;
        let payload = Blob::new(json);
        let res = self.client.invoke()
            .function_name(function_name)
            .invocation_type(InvocationType::RequestResponse)
            .payload(payload)
            .send().await?;
        if let Some(payload) = res.payload {
            let bytes = payload.into_inner();
            let response: Response<String, String> = from_slice(&bytes)?;
            let hash = std::result::Result::<String, String>::from(response)?;
            return Ok(hash)
        }
        Err("internal server Error")?
    }

    pub async fn verify(&self, password: String, hash: String) -> Result<()> {
        let function_name = var("ARGON").unwrap_or(String::from("Argon"));
        let request = Request::Verify(password, hash);
        let json = json(&request)?;
        let payload = Blob::new(json);
        let res = self.client.invoke()
            .function_name(function_name)
            .invocation_type(InvocationType::RequestResponse)
            .payload(payload)
            .send().await?;
        if let Some(payload) = res.payload {
            let bytes = payload.into_inner();
            let response: Response<Option<()>, String> = from_slice(&bytes)?;
            std::result::Result::<Option<()>, String>::from(response)?;
            return Ok(())
        }
        Err("internal server Error")?
    }
}