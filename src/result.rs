use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub enum Error {
    Io(io::ErrorKind, &'static str),
    Http(ureq::ErrorKind, &'static str),
    Arg(clap::ErrorKind, &'static str),
    Db(sled::Error, &'static str),
}

impl Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err.kind(), "io error")
    }
}

impl From<ureq::Error> for Error {
    fn from(err: ureq::Error) -> Self {
        Error::Http(err.kind(), "http error")
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        let k = io::Error::from(err);
        Error::Io(k.kind(), "serde error")
    }
}

impl From<clap::Error> for Error {
    fn from(err: clap::Error) -> Self {
        Error::Arg(err.kind(), "clap error")
    }
}

impl From<sled::Error> for Error {
    fn from(err: sled::Error) -> Self {
        Error::Db(err, "sled error")
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref kind, ref reason) => write!(f, "IO error: ({:?}, {})", kind, reason),
            Error::Http(ref kind, ref reason) => write!(f, "HTTP error: ({:?}, {})", kind, reason),
            Error::Arg(ref kind, ref reason) => {
                write!(f, "Parsing Argument error: {:?}, {}", kind, reason)
            }
            Error::Db(ref kind, ref reason) => {
                write!(f, "sled error: {:?} {}", kind, reason)
            }
        }
    }
}
