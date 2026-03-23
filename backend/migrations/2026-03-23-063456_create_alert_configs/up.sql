-- Your SQL goes here
-- ── Configurações de alerta ───────────────────────────────────
-- ── Configurações de alerta ───────────────────────────────────
CREATE TABLE alert_configs (
    id                 UUID    PRIMARY KEY DEFAULT gen_random_uuid(),
    threshold_pct      SMALLINT NOT NULL DEFAULT 80,
    notify_browser     BOOLEAN  NOT NULL DEFAULT TRUE,
    notify_email       BOOLEAN  NOT NULL DEFAULT FALSE,
    email_recipient    VARCHAR(200),
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
 
-- Configuração padrão
INSERT INTO alert_configs (threshold_pct, notify_browser) VALUES (80, TRUE);
 
