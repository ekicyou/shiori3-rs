#[derive(Debug, PartialEq)]
pub enum ApiError {
    NotLoad,
    PoisonError,
    EventSendError,
    EventNotInitialized,
    EventCanceled,
    Shutdowned,
}

pub type ApiResult<T> = std::result::Result<T, ApiError>;
