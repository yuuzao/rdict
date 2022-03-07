use std::{fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(io::ErrorKind, &'static str),
    Http(ureq::ErrorKind, &'static str),
    Arg(clap::ErrorKind, &'static str),
    Db(sled::Error, &'static str),
    Audio(AudioError, &'static str),
}

#[derive(Debug)]
pub enum AudioError {
    Play(rodio::PlayError),
    Stream(rodio::StreamError),
    Device(rodio::DevicesError),
    Decode(rodio::decoder::DecoderError),
    Mp3(mp3_duration::ErrorKind),
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

impl From<rodio::PlayError> for Error {
    fn from(err: rodio::PlayError) -> Self {
        Error::Audio(AudioError::Play(err), "audio erorr")
    }
}

impl From<rodio::StreamError> for Error {
    fn from(err: rodio::StreamError) -> Self {
        Error::Audio(AudioError::Stream(err), "audio erorr")
    }
}

impl From<rodio::DevicesError> for Error {
    fn from(err: rodio::DevicesError) -> Self {
        Error::Audio(AudioError::Device(err), "audio erorr")
    }
}

impl From<rodio::decoder::DecoderError> for Error {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        Error::Audio(AudioError::Decode(err), "audio erorr")
    }
}

impl From<mp3_duration::MP3DurationError> for Error {
    fn from(err: mp3_duration::MP3DurationError) -> Self {
        Error::Audio(AudioError::Mp3(err.kind), "audio error")
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
            Error::Audio(ref kind, ref reason) => {
                write!(f, "audio error: {:?} {}", kind, reason)
            }
        }
    }
}
