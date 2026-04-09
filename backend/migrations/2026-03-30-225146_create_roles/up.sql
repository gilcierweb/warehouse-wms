-- Your SQL goes here
-- -- Roles 
CREATE TABLE roles (
    id              UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name            VARCHAR NOT NULL,
    resource_type   VARCHAR,
    resource_id     UUID,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_roles_name_resource ON roles (name, resource_type, resource_id);
