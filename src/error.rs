use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum MpqError {
    NoHeader,
    IoError(IoError),
    UnsupportedVersion,
    Corrupted,
    FileNotFound,
    UnsupportedCompression { kind: String },
}

impl fmt::Display for MpqError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MpqError::NoHeader => write!(f, "No header found"),
            MpqError::IoError(err) => write!(f, "IO Error: {err}"),
            MpqError::UnsupportedVersion => write!(f, "Unsupported MPQ version"),
            MpqError::Corrupted => write!(f, "Corrupted archive"),
            MpqError::FileNotFound => write!(f, "File not found"),
            MpqError::UnsupportedCompression { kind } => {
                write!(f, "Compression type unsupported: {kind}")
            }
        }
    }
}

impl Error for MpqError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MpqError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<IoError> for MpqError {
    fn from(err: IoError) -> Self {
        MpqError::IoError(err)
    }
}
