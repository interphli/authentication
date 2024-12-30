pub mod verification;
pub mod user;


use super::types::Error;


type Result<T> = std::result::Result<T, Error>;