use crate::prelude::*;

pub static NO_CONTENT_204: &'static str = include_str!("204.txt");

/// 500 Internal Server Error を作成します。
pub fn server_error(err: ApiError) -> String {
    match err {
        ApiError::NoContent => NO_CONTENT_204.to_owned(),
        _ => {
            let mut buf = include_str!("500.txt").to_owned();
            buf.push_str(&format!("X-ERROR-REASON: {}\r\n\r\n", err));
            buf
        }
    }
}

/// 200 OK を作成します。
pub fn ok(value: &str) -> String {
    let mut buf = include_str!("200.txt").to_owned();
    buf.push_str(&format!("Value: {}\r\n\r\n", value));
    buf
}

#[test]
fn response_test() {
    {
        let value = "何か喋った。";
        assert_eq!(ok(value), include_str!("200-Test.txt"));
    }
    assert_eq!(
        server_error(ApiError::Poison),
        include_str!("500-Poison.txt")
    );
    assert_eq!(server_error(ApiError::NoContent), include_str!("204.txt"));
}
