use std::io;

// TODO Use thiserror
#[derive(Debug)]
pub enum ShellDeckErrorKind {
    IoError(io::Error),
    ParseError(serde_json::Error),
    RegedError(regex::Error),
    DialoguerError(dialoguer::Error),
    FailedToExecute,
}

impl From<io::Error> for ShellDeckErrorKind {
    fn from(err: io::Error) -> Self {
        ShellDeckErrorKind::IoError(err)
    }
}

impl From<serde_json::Error> for ShellDeckErrorKind {
    fn from(err: serde_json::Error) -> Self {
        ShellDeckErrorKind::ParseError(err)
    }
}

impl From<regex::Error> for ShellDeckErrorKind {
    fn from(err: regex::Error) -> Self {
        ShellDeckErrorKind::RegedError(err)
    }
}

impl From<dialoguer::Error> for ShellDeckErrorKind {
    fn from(err: dialoguer::Error) -> Self {
        ShellDeckErrorKind::DialoguerError(err)
    }
}