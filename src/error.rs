use super::parsers;
use std::str::Utf8Error;
use std::sync::PoisonError;
use thiserror::Error;

pub type MyResult<T> = Result<T, MyError>;

#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub enum MyError {
    #[error("others error")]
    Others,

    #[error("load error")]
    Load,

    #[error("not initialized error")]
    NotInitialized,

    #[error("Poison error")]
    Poison,

    #[error("Shiori request parse error for '{0}'")]
    ParseRequest(parsers::req::ParseError),

    #[error("ANSI encodeing error")]
    EncodeAnsi,
    #[error("UTF8 encodeing error")]
    EncodeUtf8(Utf8Error),

    #[error("script error: {}", message)]
    Script { message: String },
}

impl From<parsers::req::ParseError> for MyError {
    fn from(error: parsers::req::ParseError) -> MyError {
        MyError::ParseRequest(error)
    }
}

impl<G> From<PoisonError<G>> for MyError {
    fn from(_error: PoisonError<G>) -> MyError {
        MyError::Poison
    }
}
impl From<Utf8Error> for MyError {
    fn from(error: Utf8Error) -> MyError {
        MyError::EncodeUtf8(error)
    }
}

impl MyError {
    #[allow(dead_code)]
    pub fn script_error(message: String) -> MyError {
        MyError::Script { message }
    }
}
