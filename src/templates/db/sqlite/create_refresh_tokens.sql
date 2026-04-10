-- Add migration script here
CREATE TABLE refresh_token (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    userId TEXT NOT NULL,
    issuedAt DATETIME,
    issuedAt DATETIME NOT NULL DEFAULT (datetime('now')),
    expiresAt DATETIME,
    usedAt DATETIME,
    jti TEXT NOT NULL,
    token TEXT NOT NULL,
    valid BOOLEAN NOT NULL DEFAULT 0,

    FOREIGN KEY (userId) REFERENCES user(id) ON DELETE CASCADE
);

CREATE INDEX idx_refresh_token_jti ON refresh_token(jti);
CREATE INDEX idx_refresh_token_userId ON refresh_token(userId);
