use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Deserialize, Serialize, PartialEq, Debug)]
pub enum RequestResultErrorKind {
    InternalFailure,
    MalformedRequest,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestResultError {
    kind: RequestResultErrorKind,
    message: String,
}

impl RequestResultError {
    pub fn new(kind: RequestResultErrorKind, message: impl Into<String>) -> RequestResultError {
        RequestResultError {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> RequestResultErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for RequestResultError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
