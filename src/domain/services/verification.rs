use super::super::types::{Verification, Error, Id};
use super::super::db::verification;
use aws_sdk_dynamodb::Client;
use chrono::TimeDelta;
use chrono::Utc;
use uuid::Uuid;
use rand::Rng;


type Result<T> = std::result::Result<T, Error>;


const EXPIRY_MINUTES: i64 = 10;

///This function generates a new verification code and saves it to the database and then returns it.
pub async fn generate_verification_code(client: &Client, user_id: Id) -> Result<Verification> {
    let code = rand::thread_rng().gen_range(100_000..1_000_000);
    let expires = Utc::now() + TimeDelta::minutes(EXPIRY_MINUTES);
    let verification = Verification {
        user_id,
        magic_id: Uuid::new_v4(),
        code: code,
        expires,
    };
    verification::create_verification(client, verification.clone()).await?;
    Ok(verification)
}


///This function gets verification with the provided magic_id.
/// if no verification is found. it returns an error of VerificationNotFound
/// if verification is found. it checks if it has expired or not. if expired it returns an error of VerificationCodeExpired.
/// else it returns the user_id of that verification.
pub async fn verify_magic_link(client: &Client, magic_id: Uuid) -> Result<Id> {
    let option = verification::get_verification_by_magic_id(client, magic_id).await?;
    match option {
        None => Err(Error::VerificationCodeNotFound),
        Some(verification) => {
            let current_time = Utc::now();
            if current_time > verification.expires {
                return Err(Error::VerificationCodeExpired)
            }
            Ok(verification.user_id)
        }
    }
}


///This functions gets the verification for the provided user_id.
///Returns an Error of VerificationNotfound if None is returned.
///if a Value is returned it compares the provided code with the stored code.
///If the comparison is true it returns `()` else and error of `WrongVerificationCode`
pub async fn verify_verification_code(client: &Client, user_id: Id, code: u32) -> Result<()> {
    let option = verification::get_verification_code_by_user(client, user_id).await?;
    match option {
        None => Err(Error::VerificationCodeNotFound),
        Some(verification) => {
            if verification.code != code {
                return Err(Error::WrongVerificationCode)
            }
            Ok(())
        }
    }
}