use super::parsers;
use failure::{Backtrace, Context, Fail};
use std::fmt;
use std::fmt::Display;
use std::str::Utf8Error;
use std::sync::PoisonError;

pub type MyResult<T> = Result<T, MyError>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum MyErrorKind {
    #[allow(dead_code)]
    #[fail(display = "others error")]
    Others,

    #[allow(dead_code)]
    #[fail(display = "load error")]
    Load,

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

    #[allow(dead_code)]
    #[fail(display = "script error: {}", message)]
    Script { message: String },
}

impl From<parsers::req::ParseError> for MyError {
    fn from(error: parsers::req::ParseError) -> MyError {
        let cp = error.clone();
        MyError {
            inner: error.context(MyErrorKind::ParseRequest(cp)),
        }
    }
}

impl<G> From<PoisonError<G>> for MyError {
    fn from(_error: PoisonError<G>) -> MyError {
        MyError::from(MyErrorKind::Poison)
    }
}
impl From<Utf8Error> for MyError {
    fn from(error: Utf8Error) -> MyError {
        MyError {
            inner: error.context(MyErrorKind::EncodeUtf8(error)),
        }
    }
}

impl MyError {
    #[allow(dead_code)]
    pub fn script_error(message: String) -> MyError {
        let kind = MyErrorKind::Script { message: message };
        MyError::from(kind)
    }
}

/* ----------- failure boilerplate ----------- */
#[derive(Debug)]
pub struct MyError {
    inner: Context<MyErrorKind>,
}

impl Fail for MyError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl MyError {
    #[allow(dead_code)]
    pub fn new(inner: Context<MyErrorKind>) -> MyError {
        MyError { inner }
    }

    #[allow(dead_code)]
    pub fn kind(&self) -> &MyErrorKind {
        self.inner.get_context()
    }
}

impl From<MyErrorKind> for MyError {
    fn from(kind: MyErrorKind) -> MyError {
        MyError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<MyErrorKind>> for MyError {
    fn from(inner: Context<MyErrorKind>) -> MyError {
        MyError { inner }
    }
}
