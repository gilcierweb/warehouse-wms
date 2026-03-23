-- Your SQL goes here
-- ── Movimentos (log de entradas/saídas) ───────────────────────
CREATE TABLE movements (
    id          UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
    slot_id     UUID REFERENCES slots(id),
    movement_type  INTEGER NOT NULL DEFAULT 1,      -- enum entry | exit
    operator_id UUID         REFERENCES users(id),
    operator_name VARCHAR(80),              -- snapshot do nome no momento
    sku         VARCHAR(100),
    note        TEXT,
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
 
    CONSTRAINT movements_type_check CHECK (movement_type IN (1,2))
);
 
CREATE INDEX idx_movements_slot_id    ON movements(slot_id);
CREATE INDEX idx_movements_created_at ON movements(created_at DESC);
CREATE INDEX idx_movements_type       ON movements(movement_type);
