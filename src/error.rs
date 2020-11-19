use crate::parsers::req;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ApiError {
    /* api */
    #[error("unimplemented code.")]
    Unimplemented,

    #[error("SHIORI/3.0 204 No Content")]
    /// SHIORI/3.0 204 No Content
    NoContent,

    #[error("shiori not loaded.")]
    NotLoad,

    #[error("thread poison.")]
    Poison,

    #[error("send error.")]
    Send,

    #[error("recv error. {0}")]
    Recv(#[from] std::sync::mpsc::RecvError),

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

    #[error("encode error:utf8 {0}")]
    EncodeUtf8(#[from] std::str::Utf8Error),

    #[error("shiori request parse error. {0}")]
    ParseError(#[from] req::ParseError),
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;

impl<T> From<std::sync::PoisonError<T>> for ApiError {
    fn from(_: std::sync::PoisonError<T>) -> Self {
        Self::Poison
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for ApiError {
    fn from(_: std::sync::mpsc::SendError<T>) -> Self {
        Self::Send
    }
}
