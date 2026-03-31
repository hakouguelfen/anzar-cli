use std::fmt;

#[derive(Debug, Default, Clone)]
pub enum AuthStrategy {
    #[default]
    Session,
    Jwt,
}

impl fmt::Display for AuthStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthStrategy::Session => write!(f, "Session"),
            AuthStrategy::Jwt => write!(f, "Jwt"),
        }
    }
}
