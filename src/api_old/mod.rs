mod api;
#[cfg(any(windows))]
mod windows;

pub use self::api::Shiori3;
#[cfg(any(windows))]
pub use self::windows::RawAPI;
