use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;
use std::sync::PoisonError;

pub type ShioriResult<T> = Result<T, Error>;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "others error")]
    Others,
    #[fail(display = "not initialized error")]
    NotInitialized,
    #[fail(display = "Poison error")]
    Poison,
    #[fail(display = "GStr error, impl({:?})", _0)]
    GStr(::hglobal::GStrError),
    #[fail(display = "IO error")]
    Io,
    #[fail(display = "Serde error")]
    Serde,
    #[fail(display = "Hyper error")]
    Hyper,
    #[fail(display = "Cannot parse uri")]
    UrlParse,
    #[fail(display = "askama error")]
    Askama,
    #[fail(display = "service error")]
    Service,
}

impl<G> From<PoisonError<G>> for Error {
    fn from(_error: PoisonError<G>) -> Error {
        Error::from(ErrorKind::Poison)
    }
}
impl From<::hglobal::GStrError> for Error {
    fn from(error: ::hglobal::GStrError) -> Error {
        Error::from(ErrorKind::GStr(error))
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
    pub fn new(inner: Context<ErrorKind>) -> Error {
        Error { inner }
    }

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
