use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum CacheDriver {
    #[default]
    MemCached,
    Redis,
}
impl fmt::Display for CacheDriver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheDriver::MemCached => write!(f, "MemCached"),
            CacheDriver::Redis => write!(f, "Redis"),
        }
    }
}
