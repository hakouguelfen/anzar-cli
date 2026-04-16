// Authentication
pub const JWT_AUTH: &str = include_str!("../templates/auth/jwt.yml");
pub const SESSION_AUTH: &str = include_str!("../templates/auth/session.yml");
pub const CONFIG_TEMPLATE: &str = include_str!("../templates/configuration.yml");

// Compose
pub const COMPOSE: &str = include_str!("../templates/compose.conf.yml");
// pub const POSTGRES_COMPOSE: &str = include_str!("../templates/compose/postgres.yml");
pub const MONGO_COMPOSE: &str = include_str!("../templates/compose/mongo.yml");
pub const REDIS: &str = include_str!("../templates/compose/redis.yml");
pub const MEMCACHED: &str = include_str!("../templates/compose/memcached.yml");

// Databases
pub const CREATE_USERS: &str = include_str!("../templates/db/sqlite/create_users.sql");
pub const CREATE_ACCOUNTS: &str = include_str!("../templates/db/sqlite/create_accounts.sql");
pub const CREATE_SESSIONS: &str = include_str!("../templates/db/sqlite/create_sessions.sql");
pub const CREATE_REFRESH_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_refresh_tokens.sql");
pub const CREATE_PASSWORD_RESET_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_password_reset_tokens.sql");
pub const CREATE_EMAIL_VERIFICATION_TOKENS: &str =
    include_str!("../templates/db/sqlite/create_email_verification_tokens.sql");

pub const SESSION_TABLES: [(&str, &str); 5] = [
    (CREATE_USERS, "anzar_create_users"),
    (CREATE_ACCOUNTS, "anzar_create_accounts"),
    (
        CREATE_PASSWORD_RESET_TOKENS,
        "anzar_create_password_reset_tokens",
    ),
    (
        CREATE_EMAIL_VERIFICATION_TOKENS,
        "anzar_create_email_verification_tokens",
    ),
    (CREATE_SESSIONS, "anzar_create_sessions"),
];

pub const JWT_TABLES: [(&str, &str); 5] = [
    (CREATE_USERS, "anzar_create_users"),
    (CREATE_ACCOUNTS, "anzar_create_accounts"),
    (
        CREATE_PASSWORD_RESET_TOKENS,
        "anzar_create_password_reset_tokens",
    ),
    (
        CREATE_EMAIL_VERIFICATION_TOKENS,
        "anzar_create_email_verification_tokens",
    ),
    (CREATE_REFRESH_TOKENS, "anzar_create_refresh_tokens"),
];
