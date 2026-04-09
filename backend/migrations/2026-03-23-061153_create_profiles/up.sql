-- Your SQL goes here
CREATE TABLE IF NOT EXISTS profiles
(
    id          UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
    -- Sensitive fields stored encrypted with pgcrypto
        first_name_enc  BYTEA,
        last_name_enc   BYTEA,
        phone_enc       BYTEA,
    full_name  BYTEA,
    nickname VARCHAR(255) NULL,
    bio TEXT NULL,
    birthday DATE NULL,
    avatar VARCHAR(255) NULL,
    phone BIGINT NULL,
    social_network  JSONB NOT NULL DEFAULT '{}',
        status          BOOLEAN NOT NULL DEFAULT TRUE,
    user_id UUID NOT NULL UNIQUE,
   
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS users_detail_id_user_id_idx ON profiles (id, user_id);