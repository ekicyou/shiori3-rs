use winapi::_core::str::Utf8Error;

#[derive(Fail, Copy, Eq, PartialEq, Clone, Debug)]
pub enum GStrError {
    #[fail(display = "ANSI encodeing error")]
    AnsiEncode,
    #[fail(display = "UTF8 encodeing error")]
    Utf8Encode(#[fail(cause)] Utf8Error),
}
impl From<Utf8Error> for GStrError {
    fn from(err: Utf8Error) -> GStrError {
        GStrError::Utf8Encode(err)
    }
}
