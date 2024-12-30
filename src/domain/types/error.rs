use aws_sdk_dynamodb::operation::put_item::PutItemError;
use std::fmt::{Display, Formatter, Debug};
use std::error::Error as StdErrorTrait;
use aws_sdk_config::error::SdkError;
use lambda_http::http::StatusCode;

pub type StdError = Box<dyn StdErrorTrait>;

#[derive(Debug)]
pub enum Error {
    UserNotFound,
    VerificationCodeNotFound,
    UserWithEmailAlreadyExists,
    InternalServerError(StdError),
    Custom(StatusCode, StdError)
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UserNotFound => write!(f, "user not found"),
            Error::VerificationCodeNotFound => write!(f, "verification code not found"),
            Error::UserWithEmailAlreadyExists => write!(f, "user with the same email exists"),
            Error::InternalServerError(err) => write!(f, "{err}"),
            Error::Custom(status, err) => write!(f, "{err}"),
        }
    }
}


impl std::error::Error for Error {}


impl<E: Into<Error> + std::error::Error + 'static, R: Debug + 'static> From<SdkError<E, R>> for Error {
    fn from(value: SdkError<E, R>) -> Self {
        match value {
            SdkError::ServiceError(err) => err.into_err().into(),
            _ => Error::InternalServerError(Box::new(value))
        }
    }
}


impl From<PutItemError> for Error {
    fn from(value: PutItemError) -> Self {
       Error::InternalServerError(Box::new(value))
    }
}