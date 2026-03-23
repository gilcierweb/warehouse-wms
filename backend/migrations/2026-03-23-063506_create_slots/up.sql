-- Your SQL goes here
-- ── Slots (posições do armazém) ───────────────────────────────
CREATE TABLE slots (
    -- PK técnica: imutável, nunca exposta ao usuário final
    id          UUID         PRIMARY KEY DEFAULT gen_random_uuid(),
 
    -- Endereço legível: "A-5-N2" — dado de negócio, único mas separado da PK
    -- Pode ser recalculado/renomeado sem quebrar nenhuma FK
    address     VARCHAR(12)  NOT NULL UNIQUE,
 
    -- Componentes individuais — úteis para filtrar por rua, lane, etc.
    street      CHAR(1)      NOT NULL,      -- A-Z
    position    SMALLINT     NOT NULL,      -- 1-30
    lane        VARCHAR(3)   NOT NULL,      -- N1 | N2 | N3
 
    status      VARCHAR(10)  NOT NULL DEFAULT 'free',  -- free | occupied
    sku         VARCHAR(100),                          -- produto atual na vaga
    updated_by  UUID         REFERENCES users(id),
    
    created_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    CONSTRAINT slots_street_check    CHECK (street BETWEEN 'A' AND 'Z'),
    CONSTRAINT slots_position_check  CHECK (position BETWEEN 1 AND 30),
    CONSTRAINT slots_lane_check      CHECK (lane IN ('N1','N2','N3')),
    CONSTRAINT slots_status_check    CHECK (status IN ('free','occupied')),
 
    -- Garante que não existam dois slots com mesma rua+posição+lane
    CONSTRAINT slots_address_unique  UNIQUE (street, position, lane)
);
 
CREATE INDEX idx_slots_address ON slots(address);   -- busca por endereço legível
CREATE INDEX idx_slots_street  ON slots(street);    -- filtro por rua
CREATE INDEX idx_slots_status  ON slots(status);    -- filtro livre/ocupado
 
-- ── Seed: layout padrão do armazém (A-F, N1-N3, posições 1-20) ──
INSERT INTO slots (id, address, street, position, lane)
SELECT
    gen_random_uuid(),
    CONCAT(s.street, '-', p.pos, '-', l.lane),  -- endereço legível como campo
    s.street,
    p.pos,
    l.lane
FROM
    (VALUES ('A'),('B'),('C'),('D'),('E'),('F')) AS s(street),
    generate_series(1, 20)                       AS p(pos),
    (VALUES ('N1'),('N2'),('N3'))                AS l(lane)
ON CONFLICT DO NOTHING;
 
 