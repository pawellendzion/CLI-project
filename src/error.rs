use std::{fmt, io::{self}, env, result, error};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: Box<ErrorKind>,
}

impl Error {
    pub fn new(kind: ErrorKind) -> Self {
        Error { kind: Box::new(kind) }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.kind, f)
    }
}

impl error::Error for Error {}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::new(ErrorKind::IO(value))
    }
}

impl From<env::VarError> for Error {
    fn from(value: env::VarError) -> Self {
        Error::new(ErrorKind::Var(Box::new(value)))
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::new(ErrorKind::SerializeDeserialize(value))
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    IO(io::Error),
    Var(Box<env::VarError>),
    SerializeDeserialize(serde_json::Error),
    NoEmptySlots,
    DoubleFree,
    Message(Box<str>),
    PathNotExists(Box<str>),
    NodeNotExists(usize),
    FreeEmptySlot,
    FreeInvalidSlot,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::IO(err) => fmt::Display::fmt(err, f),
            ErrorKind::Var(err) => fmt::Display::fmt(err, f),
            ErrorKind::SerializeDeserialize(err) => fmt::Display::fmt(err, f),
            ErrorKind::NoEmptySlots => todo!(),
            ErrorKind::DoubleFree => todo!(),
            ErrorKind::Message(msg) => f.write_str(msg),
            ErrorKind::PathNotExists(path) => write!(f, r#"Path "{}" does not exist"#, path),
            ErrorKind::NodeNotExists(id) => write!(f, r#"Node with id = {} does not exist"#, id),
            ErrorKind::FreeInvalidSlot => f.write_str("Attempt to free invalid slot"),
            ErrorKind::FreeEmptySlot => f.write_str("Attempt to free empty slot"),
        }
    }
}
