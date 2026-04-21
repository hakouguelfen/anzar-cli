use crate::shared::configuration::DatabaseDriver;

// Authentication
pub const JWT_AUTH: &str = include_str!("../templates/auth/jwt.yml");
pub const SESSION_AUTH: &str = include_str!("../templates/auth/session.yml");
pub const CONFIG_TEMPLATE: &str = include_str!("../templates/configuration.yml");

// Compose
pub const COMPOSE: &str = include_str!("../templates/compose.conf.yml");
pub const POSTGRES_COMPOSE: &str = include_str!("../templates/compose/postgres.yml");
pub const MONGO_COMPOSE: &str = include_str!("../templates/compose/mongo.yml");
pub const REDIS: &str = include_str!("../templates/compose/redis.yml");
pub const MEMCACHED: &str = include_str!("../templates/compose/memcached.yml");

// Databases
// -- Sqlite
pub const SQLITE_CREATE_USERS: &str = include_str!("../templates/db/sqlite/create_users.sql");
pub const SQLITE_CREATE_ACCOUNTS: &str = include_str!("../templates/db/sqlite/create_accounts.sql");
pub const SQLITE_CREATE_SESSIONS: &str = include_str!("../templates/db/sqlite/create_sessions.sql");
pub const SQLITE_CREATE_REFRESH_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_refresh_tokens.sql");
pub const SQLITE_CREATE_PASSWORD_RESET_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_password_reset_tokens.sql");
pub const SQLITE_CREATE_EMAIL_VERIFICATION_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_email_verification_tokens.sql");

// -- PostgreSQL
pub const PG_CREATE_USERS: &str = include_str!("../templates/db/postgres/create_users.sql");
pub const PG_CREATE_ACCOUNTS: &str = include_str!("../templates/db/postgres/create_accounts.sql");
pub const PG_CREATE_SESSIONS: &str = include_str!("../templates/db/postgres/create_sessions.sql");
pub const PG_CREATE_REFRESH_TOKENS: &str =
    include_str!("../templates/db/postgres/create_refresh_tokens.sql");
pub const PG_CREATE_PASSWORD_RESET_TOKENS: &str =
    include_str!("../templates/db/postgres/create_password_reset_tokens.sql");
pub const PG_CREATE_EMAIL_VERIFICATION_TOKENS: &str =
    include_str!("../templates/db/postgres/create_email_verification_tokens.sql");

pub const fn session_tables(db: DatabaseDriver) -> [(&'static str, &'static str); 5] {
    match db {
        DatabaseDriver::PostgreSQL => [
            (PG_CREATE_USERS, "anzar_create_users"),
            (PG_CREATE_ACCOUNTS, "anzar_create_accounts"),
            (PG_CREATE_SESSIONS, "anzar_create_sessions"),
            (
                PG_CREATE_PASSWORD_RESET_TOKENS,
                "anzar_create_password_reset_tokens",
            ),
            (
                PG_CREATE_EMAIL_VERIFICATION_TOKENS,
                "anzar_create_email_verification_tokens",
            ),
        ],
        DatabaseDriver::SQLite => [
            (SQLITE_CREATE_USERS, "anzar_create_users"),
            (SQLITE_CREATE_ACCOUNTS, "anzar_create_accounts"),
            (SQLITE_CREATE_SESSIONS, "anzar_create_sessions"),
            (
                SQLITE_CREATE_PASSWORD_RESET_TOKENS,
                "anzar_create_password_reset_tokens",
            ),
            (
                SQLITE_CREATE_EMAIL_VERIFICATION_TOKENS,
                "anzar_create_email_verification_tokens",
            ),
        ],
        DatabaseDriver::MongoDB => todo!(),
    }
}

pub const fn jwt_tables(db: DatabaseDriver) -> [(&'static str, &'static str); 5] {
    match db {
        DatabaseDriver::PostgreSQL => [
            (PG_CREATE_USERS, "anzar_create_users"),
            (PG_CREATE_ACCOUNTS, "anzar_create_accounts"),
            (PG_CREATE_REFRESH_TOKENS, "anzar_create_refresh_tokens"),
            (
                PG_CREATE_PASSWORD_RESET_TOKENS,
                "anzar_create_password_reset_tokens",
            ),
            (
                PG_CREATE_EMAIL_VERIFICATION_TOKENS,
                "anzar_create_email_verification_tokens",
            ),
        ],
        DatabaseDriver::SQLite => [
            (SQLITE_CREATE_USERS, "anzar_create_users"),
            (SQLITE_CREATE_ACCOUNTS, "anzar_create_accounts"),
            (SQLITE_CREATE_REFRESH_TOKENS, "anzar_create_refresh_tokens"),
            (
                SQLITE_CREATE_PASSWORD_RESET_TOKENS,
                "anzar_create_password_reset_tokens",
            ),
            (
                SQLITE_CREATE_EMAIL_VERIFICATION_TOKENS,
                "anzar_create_email_verification_tokens",
            ),
        ],
        DatabaseDriver::MongoDB => todo!(),
    }
}
