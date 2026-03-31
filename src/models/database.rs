use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum DatabaseDriver {
    #[default]
    MongoDB,
    PostgreSQL,
    SQLite,
}
impl fmt::Display for DatabaseDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseDriver::MongoDB => write!(f, "MongoDB"),
            DatabaseDriver::PostgreSQL => write!(f, "PostgreSQL"),
            DatabaseDriver::SQLite => write!(f, "SQLite"),
        }
    }
}
