use aws_sdk_dynamodb::operation::batch_get_item::BatchGetItemError;
use aws_sdk_dynamodb::operation::put_item::PutItemError;
use aws_sdk_dynamodb::operation::get_item::GetItemError;
use lambda_http::http::header::CONTENT_TYPE;
use lambda_http::http::header::HeaderValue;
use std::fmt::{Display, Formatter, Debug};
use aws_sdk_dynamodb::error::BuildError;
use std::error::Error as StdErrorTrait;
use aws_sdk_config::error::SdkError;
use lambda_http::http::StatusCode;
use lambda_http::Response;
use lambda_http::Body;

pub type StdError = Box<dyn StdErrorTrait>;

#[derive(Debug)]
pub enum Error {
    UserNotFound,
    UserWithEmailAlreadyExists,
    VerificationCodeNotFound,
    VerificationCodeExpired,
    WrongVerificationCode,
    InternalServerError(StdError),
    Custom(StatusCode, String, StdError)
}


impl Error {
    fn as_json(&self) -> (StatusCode, String) {
        use Error::*;
        match self {
            UserNotFound => (StatusCode::NOT_FOUND, String::from("user not found")),
            UserWithEmailAlreadyExists => (StatusCode::CONFLICT, String::from("user with this email already exists")),
            VerificationCodeNotFound => (StatusCode::NOT_FOUND, String::from("verification-code not found")),
            VerificationCodeExpired => (StatusCode::GONE, String::from("the verification-code has expired")),
            WrongVerificationCode => (StatusCode::BAD_REQUEST, String::from("wrong verification code")),
            InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, String::from("internal server error. We are working on resolving the problem")),
            Custom(status, msg, _) => (*status, msg.clone())
        }
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UserNotFound => write!(f, "user not found"),
            Error::UserWithEmailAlreadyExists => write!(f, "user with the same email exists"),
            Error::VerificationCodeNotFound => write!(f, "verification code not found"),
            Error::VerificationCodeExpired => write!(f, "verification code has expired"),
            Error::WrongVerificationCode => write!(f, "wrong verification code"),
            Error::InternalServerError(err) => write!(f, "{err}"),
            Error::Custom(status, _, err) => write!(f, "{err}"),
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


impl From<GetItemError> for Error {
    fn from(value: GetItemError) -> Self {
        Error::InternalServerError(Box::new(value))
    }
}


impl From<StdError> for Error {
    fn from(value: StdError) -> Self {
        Error::InternalServerError(value)
    }
}


impl From<BatchGetItemError> for Error {
    fn from(value: BatchGetItemError) -> Self {
        Error::InternalServerError(Box::new(value))
    }
}


impl From<BuildError> for Error {
    fn from(value: BuildError) -> Self {
        Error::InternalServerError(Box::new(value))
    }
}


impl From<Error> for Response<Body> {
    fn from(err: Error) -> Self {
        let (status, msg) = err.as_json();
        let body  = Body::Text(format!("{{\"msg\": \"{}\"}}", msg));
        let mut res = Response::new(body);
        res.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        *res.status_mut() = status;
        res
    }
}