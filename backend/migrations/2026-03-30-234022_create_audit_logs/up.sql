-- Your SQL goes here
-- AUDIT LOGS (append-only, no updates) 
CREATE TABLE audit_logs (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id         UUID REFERENCES users(id) ON DELETE SET NULL,
    action          VARCHAR(100) NOT NULL,
    resource_type   VARCHAR(100),
    resource_id     UUID,
    ip_address      VARCHAR(45),
    user_agent      TEXT,
    metadata        JSONB,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_user      ON audit_logs (user_id, created_at DESC);
CREATE INDEX idx_audit_action    ON audit_logs (action, created_at DESC);
CREATE INDEX idx_audit_resource  ON audit_logs (resource_type, resource_id);
