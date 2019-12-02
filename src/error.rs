use crate::parsers::req;
use std::str;

#[derive(Debug, PartialEq)]
pub enum ApiError {
    /* api */
    NotLoad,
    PoisonError,
    EventSendError,
    EventNotInitialized,
    EventCanceled,
    Shutdowned,

    EncodeAnsi,
    EncodeUtf8,

    ParseError(req::ParseError),
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl From<str::Utf8Error> for ApiError {
    fn from(_: str::Utf8Error) -> ApiError {
        ApiError::EncodeUtf8
    }
}

impl From<req::ParseError> for ApiError {
    fn from(error: req::ParseError) -> ApiError {
        ApiError::ParseError(error.clone())
    }
}
