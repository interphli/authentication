use std::error::Error as StdError;


mod config;
mod domain;
mod server;


pub type Result<T> = std::result::Result<T, Box<dyn StdError>>;


#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    Ok(())
}
