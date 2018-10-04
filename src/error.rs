use super::parsers;
use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;
use std::str::Utf8Error;
use std::sync::PoisonError;

pub type ShioriResult<T> = Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[allow(dead_code)]
    #[fail(display = "others error")]
    Others,

    #[fail(display = "not initialized error")]
    NotInitialized,

    #[fail(display = "Poison error")]
    Poison,

    #[fail(display = "Shiori request parse error")]
    ParseRequest(#[fail(cause)] parsers::req::ParseError),

    #[fail(display = "ANSI encodeing error")]
    EncodeAnsi,
    #[fail(display = "UTF8 encodeing error")]
    EncodeUtf8(#[fail(cause)] Utf8Error),
}

impl From<parsers::req::ParseError> for Error {
    fn from(error: parsers::req::ParseError) -> Error {
        let cp = error.clone();
        Error {
            inner: error.context(ErrorKind::ParseRequest(cp)),
        }
    }
}

impl<G> From<PoisonError<G>> for Error {
    fn from(_error: PoisonError<G>) -> Error {
        Error::from(ErrorKind::Poison)
    }
}
impl From<Utf8Error> for Error {
    fn from(error: Utf8Error) -> Error {
        Error {
            inner: error.context(ErrorKind::EncodeUtf8(error)),
        }
    }
}

/* ----------- failure boilerplate ----------- */
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl Error {
    #[allow(dead_code)]
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

    #[allow(dead_code)]
    pub fn kind(&self) -> &ErrorKind {
        self.inner.get_context()
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<ErrorKind>> for Error {
    fn from(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }
}
