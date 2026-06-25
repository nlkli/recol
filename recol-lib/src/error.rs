use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidLength {
        src: String,
        expected: usize,
        got: usize,
    },
    InvalidHex(String),
    InvalidUtf8(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength { src, expected, got } => {
                write!(
                    f,
                    "{src}: invalid slice length: expected {expected}, got {got}"
                )
            }
            Self::InvalidHex(s) => {
                write!(f, "invalid hex digits in '{s}'")
            }
            Self::InvalidUtf8(s) => {
                write!(f, "invalid UTF-8: {s}")
            }
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8(err.to_string())
    }
}
