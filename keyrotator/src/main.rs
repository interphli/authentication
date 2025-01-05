use aws_sdk_ssm::types::ParameterType;
use lambda_runtime::{service_fn, run};
use std::error::Error as StdError;
use lambda_runtime::LambdaEvent;
use ed25519_dalek::SigningKey;
use chrono::{Utc, TimeDelta};
use aws_sdk_ssm::Client;
use serde::Deserialize;
use rand::rngs::OsRng;
use std::env::var;
use shared::*;


type Result<T> = std::result::Result<T, Box<dyn StdError + Send + Sync>>;

#[derive(Debug, Clone, Deserialize, Copy)]
pub enum Command {
    Create,
    Update,
}


#[tokio::main]
async fn main() -> Result<()> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_ssm::Client::new(&config);
    let handler = service_fn(|event|handler(event, &client));
    run(handler).await?;
    Ok(())
}


async fn handler(event: LambdaEvent<Command>, client: &Client) -> Result<()> {
    match event.payload {
        Command::Create => create(client).await,
        Command::Update => update(client).await,
    }
}


async fn create(client: &Client) -> Result<()> {
    let keys = keys(None);
    put_key(client, &keys).await
}


async fn update(client: &Client) -> Result<()> {
    let previous_keys = match get_key(client).await {
        Ok(Some(value)) => value,
        _ => {
            let keys = keys(None);
            put_key(client, &keys).await?;
            keys
        }
    };
    if previous_keys.expires > Utc::now() + TimeDelta::days(1) {
        return  Ok(());
    }
    let prev_public_key = Some(previous_keys.public_key);
    let keys = keys(prev_public_key);
    put_key(client, &keys).await
}



fn keys(prev_public_key: Option<[u8; 32]>) -> Keys {
    let (private_key, public_key) = generate_keys();
    let expiry_days = var("EXPIRY_DAYS").unwrap_or("30".into()).parse().unwrap_or(30);
    let created_time = Utc::now();
    let expires = created_time + TimeDelta::days(expiry_days);
    Keys{private_key, public_key, prev_public_key, created_time, expires}
}


async fn get_key(client: &Client) -> Result<Option<Keys>> {
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

async fn put_key(client: &Client, keys: &Keys) -> Result<()> {
    let json = serde_json::to_string(keys)?;
    let name = PARAMETER_NAME;
    let r#type = ParameterType::SecureString;
    let _ = client.put_parameter().name(name).r#type(r#type).value(json).send().await?;
    Ok(())
}


fn generate_keys() -> ([u8; 32], [u8; 32]) {
    let keys = SigningKey::generate(&mut OsRng);
    let keys = (keys.to_bytes(), keys.verifying_key().to_bytes());
    keys
}