use std::io;
use std::result;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    None,
    Io(io::Error),
    HomeAssistant(homeassistant::Error),
    Format(fmt::Error)
}

pub type Result<S> = result::Result<S, Error>;

impl From<()> for Error {
    fn from(_err: ()) -> Error {
        Error::None
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<homeassistant::Error> for Error {
    fn from(err: homeassistant::Error) -> Error {
        Error::HomeAssistant(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Error {
        Error::Format(err)
    }
}