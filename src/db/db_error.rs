use rusqlite::Error as RusqliteError;
use std::{error::Error, fmt};

pub enum DbError {
    Sqlite(RusqliteError),
    Simple(String),
}

impl fmt::Debug for DbError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DbError::Sqlite(err) => write!(f, "Database error: {:?}", err),
            DbError::Simple(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::Sqlite(err) => err.fmt(f),
            DbError::Simple(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for DbError {}

impl From<RusqliteError> for DbError {
    fn from(err: RusqliteError) -> DbError {
        DbError::Sqlite(err)
    }
}

impl From<String> for DbError {
    fn from(msg: String) -> DbError {
        DbError::Simple(msg)
    }
}
