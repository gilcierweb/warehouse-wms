-- Your SQL goes here
-- -- Users 
CREATE TABLE users (
    id                      UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email                   VARCHAR NOT NULL DEFAULT '',
    password_hash           VARCHAR NOT NULL DEFAULT '',

    -- Recoverable
    reset_password_token    VARCHAR,
    reset_password_sent_at  TIMESTAMPTZ,

    -- Rememberable
    remember_created_at     TIMESTAMPTZ,

    -- Trackable
    sign_in_count           INTEGER NOT NULL DEFAULT 0,
    current_sign_in_at      TIMESTAMPTZ,
    last_sign_in_at         TIMESTAMPTZ,
    current_sign_in_ip      VARCHAR,
    last_sign_in_ip         VARCHAR,

    -- Confirmable
    confirmation_token      VARCHAR,
    confirmed_at            TIMESTAMPTZ,
    confirmation_sent_at    TIMESTAMPTZ,
    unconfirmed_email       VARCHAR,

    -- Lockable
    failed_attempts         INTEGER NOT NULL DEFAULT 0,
    unlock_token            VARCHAR,
    locked_at               TIMESTAMPTZ,

    -- 2FA
    totp_secret             VARCHAR,
    totp_enabled            BOOLEAN NOT NULL DEFAULT FALSE,

    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at              TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_users_email               ON users (email);
CREATE UNIQUE INDEX idx_users_reset_token         ON users (reset_password_token) WHERE reset_password_token IS NOT NULL;
CREATE UNIQUE INDEX idx_users_confirmation_token  ON users (confirmation_token) WHERE confirmation_token IS NOT NULL;
CREATE UNIQUE INDEX idx_users_unlock_token        ON users (unlock_token) WHERE unlock_token IS NOT NULL;
