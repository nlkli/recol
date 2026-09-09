use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(String),
    InvalidLength {
        src: String,
        expected: usize,
        got: usize,
    },
    InvalidHex(String),
    InvalidUtf8(String),
    Ffmpeg(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => {
                write!(f, "I/O error: {err}")
            }
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
            Self::Ffmpeg(err) => {
                write!(f, "ffmpeg failed: {err}")
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::InvalidUtf8(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
