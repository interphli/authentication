use aws_sdk_config::error::SdkError;
use lambda_http::http::StatusCode;
use lambda_http::Response;


pub trait ResponseError {
    type Body;
    fn response(&self) -> Response<Self::Body>;
}



impl<E: ResponseError<Body = String>, R> ResponseError for SdkError<E, R> {
    type Body = String;
    fn response(&self) -> Response<String> {
        match self {
            SdkError::ServiceError(err) => err.err().response(),
            _ => {
                let mut res = Response::new(String::new());
                let status = res.status_mut();
                *status = StatusCode::INTERNAL_SERVER_ERROR;
                res
            }
        }
    }
}