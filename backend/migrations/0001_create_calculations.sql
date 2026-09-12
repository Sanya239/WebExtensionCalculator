CREATE TABLE calculations (
    id          BIGSERIAL PRIMARY KEY,
    expression  TEXT NOT NULL,
    result      DOUBLE PRECISION,
    error_type  TEXT,
    message     TEXT,
    position    INTEGER,
    timestamp   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CHECK (
        (result IS NOT NULL AND error_type IS NULL AND message IS NULL AND position IS NULL)
        OR
        (result IS NULL AND error_type IS NOT NULL AND message IS NOT NULL AND position IS NOT NULL)
    )
);