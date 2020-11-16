use crate::parsers::req;
use std::str;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    /* api */
    #[error("shiori not loaded.")]
    NotLoad,

    #[error("thread poison.")]
    PoisonError,

    #[error("event send error.")]
    EventSendError,

    #[error("event not initialized.")]
    EventNotInitialized,

    #[error("event calceled.")]
    EventCanceled,

    #[error("allready shutdowned.")]
    Shutdowned,

    /* api response */
    #[error("response not received.")]
    EventResponseNotReceived,

    /* encode */
    #[error("encode error:ansi")]
    EncodeAnsi,

    #[error("encode error:utf8")]
    EncodeUtf8(#[from] std::str::Utf8Error),

    #[error("shiori request parse error.")]
    ParseError(#[from] req::ParseError),
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
