use std::fmt;
use std::io;

use clap::Args;
use serde::{Deserialize, Serialize};


#[derive(Args, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Link {
    name: String,
    pub upstream: String,
    pub downstream: String,
}

impl fmt::Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} -> {}", self.name, self.upstream, self.downstream)
    }
}

impl Link {
    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn new(name: String, upstream: String, downstream: String) -> Link {
        Link {
            name,
            upstream,
            downstream,
        }
    }
}

pub enum Error {
    Fs,
    Parse,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Fs => write!(f, "a filesystem error occured."),
            Error::Parse => write!(f, "a TOML parsing error occured"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(_error: io::Error) -> Self {
        Self::Fs
    }
}

impl From<toml::ser::Error> for Error {
    fn from(_value: toml::ser::Error) -> Self {
        Self::Parse   
    }
}
