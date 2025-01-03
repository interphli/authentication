use lambda_runtime::{service_fn, run};
use std::error::Error as StdError;
use argon::{handler, new_argon2};

mod argon;


type Result<T> = std::result::Result<T, Box<dyn StdError + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let argon2 = new_argon2().await;
    let handler = service_fn(|event|handler(event, &argon2));
    run(handler).await?;
    Ok(())
}