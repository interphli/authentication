use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::{SaltString, PasswordHash}};
use shared::{Request, Result as Response};
use lambda_runtime::{LambdaEvent, Error};
pub use argon::new_argon2;
use rand_core::OsRng;


mod argon;


pub async fn handler(event: LambdaEvent<Request>, argon2: &Argon2<'static>) -> Result<Response<Option<String>, String>, Error> {
    match event.payload {
        Request::Hash(password) => {
            let salt = SaltString::generate(OsRng);
            match argon2.hash_password(password.as_bytes(), &salt) {
                Ok(value) => Ok(Response::Ok(Some(value.to_string()))),
                Err(err) => Ok(Response::Err(err.to_string()))
            }
        },
        Request::Verify(password, hash) => {
            let hash = match PasswordHash::new(&hash) {
                Ok(value) => value,
                Err(err) => return Ok(Response::Err(err.to_string()))
            };
            match argon2.verify_password(password.as_bytes(), &hash) {
                Ok(_) => Ok(Response::Ok(None)),
                Err(err) => Ok(Response::Err(err.to_string()))
            }
        }
    }
}