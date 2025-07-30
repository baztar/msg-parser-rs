use std::io;

use thiserror::Error;

// DataTypeError is used when decode fails in datatype.rs
#[derive(Error, Debug)]
pub enum DataTypeError {
    #[error("Unknown value encoding: 0x{0}")]
    UnknownCode(String),
    #[error("Unable to decode bytes into UTF-8 string {0}")]
    Utf8Err(#[from] std::string::FromUtf8Error),
    #[error("Unable to decode bytes into UTF-16 string {0}")]
    Utf16Err(#[from] std::string::FromUtf16Error),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Datatype error: {0}")]
    DataTypeError(#[from] DataTypeError),

    #[error("Unable to read file")]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("Error parsing file with ole: {}", .source)]
    OleError {
        #[from]
        source: crate::ole::Error,
    },

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
}
