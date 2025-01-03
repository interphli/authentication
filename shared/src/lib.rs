#[derive(Debug, Clone)]
#[cfg_attr(feature = "server", derive(serde::Deserialize))]
#[cfg_attr(feature = "client", derive(serde::Serialize))]
pub enum Request {
    Hash(String),
    Verify(String, String)
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "server", derive(serde::Serialize))]
#[cfg_attr(feature = "client", derive(serde::Deserialize))]
pub enum Result<T, E> {
    Ok(T),
    Err(E)
}


impl<T, E> From<Result<T, E>> for std::result::Result<T, E> {
    fn from(result: Result<T, E>) -> Self {
        match result {
            Result::Ok(value) => Ok(value),
            Result::Err(err) => Err(err)
        }
    }
}