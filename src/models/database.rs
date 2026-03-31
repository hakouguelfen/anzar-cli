use std::fmt;

#[derive(Debug, Default, Clone)]
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
