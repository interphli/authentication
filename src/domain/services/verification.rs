use super::super::types::{Verification, Error, Id, Uuid, Either, Value, User};
use std::collections::HashMap;
use aws_sdk_dynamodb::Client;
use super::table::Table;
use chrono::TimeDelta;
use chrono::Utc;
use rand::Rng;


type Result<T> = std::result::Result<T, Error>;


pub trait VerificationService {
    const EXPIRY_MINUTES: i64 = 10;

    /// Generates a new verification code and saves it to the database, then returns it.
    async fn generate_verification_code(client: &Client, user_id: Id) -> Result<Verification> {
        let code = rand::thread_rng().gen_range(100_000..1_000_000);
        let expires = Utc::now() + TimeDelta::minutes(Self::EXPIRY_MINUTES);
        let verification = Verification {
            user_id,
            magic_id: Uuid::new_v4(),
            code: code,
            expires,
        };
        <Verification as Table>::create_item(client, verification.clone()).await?;
        Ok(verification)
    }

    /// Gets verification with the provided magic_id.
    /// If no verification is found, it returns an error of VerificationNotFound.
    /// If verification is found, it checks if it has expired or not. If expired, it returns an error of VerificationCodeExpired.
    /// Else, it returns the user_id of that verification.
    async fn verify_magic_link(client: &Client, magic_id: Uuid) -> Result<Id> {
        let key = Either::Left(magic_id);
        let option = <Verification as Table>::get_item(client, key).await?;
        match option {
            None => Err(Error::VerificationCodeNotFound),
            Some(verification) => {
                let current_time = Utc::now();
                if current_time > verification.expires {
                    return Err(Error::VerificationCodeExpired);
                }
                Ok(verification.user_id)
            }
        }
    }

    /// Gets the verification for the provided user_id.
    /// Returns an Error of VerificationNotFound if None is returned.
    /// If a Value is returned, it compares the provided code with the stored code.
    /// If the comparison is true, it returns `()`, else an error of `WrongVerificationCode`.
    async fn verify_verification_code(client: &Client, user_id: Id, code: u32) -> Result<()> {
        let key = Either::Right(user_id);
        let option = <Verification as Table>::get_item(client, key).await?;
        match option {
            None => Err(Error::VerificationCodeNotFound),
            Some(verification) => {
                if verification.code != code {
                    return Err(Error::WrongVerificationCode);
                }
                Ok(())
            }
        }
    }

    async fn verify_email(client: &Client, verification: Either<Uuid, (Id, u32)>) -> Result<User> {
        let pk = match verification {
            Either::Right(magic_id) => Self::verify_magic_link(client, magic_id).await?,
            Either::Left((user_id, code)) => {Self::verify_verification_code(client, user_id.clone(), code).await?;user_id}
        };
        let update = HashMap::from([(String::from("email_verified"), Value::Bool(true))]);
        <User as Table>::update_item(client, pk, update).await
    }
}



impl VerificationService for Verification {}