-- Add migration script here
CREATE TABLE email_verification_token (
    id TEXT PRIMARY KEY DEFAULT (lower(hex(randomblob(16)))),
    userId TEXT NOT NULL,
    issuedAt DATETIME NOT NULL DEFAULT (datetime('now')),
    expiresAt DATETIME,
    usedAt DATETIME,
    token TEXT NOT NULL,
    valid BOOLEAN NOT NULL DEFAULT 0,

    FOREIGN KEY (userId) REFERENCES user(id) ON DELETE CASCADE
);

CREATE INDEX idx_email_verification_token_token ON email_verification_token(token);
CREATE INDEX idx_email_verification_token_userId ON email_verification_token(userId);
