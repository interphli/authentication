use std::error::Error as StdError;

mod shared;
mod argon;


type Result<T> = std::result::Result<T, Box<dyn StdError>>;

#[tokio::main]
async fn main() -> Result<()> {
    Ok(())
}