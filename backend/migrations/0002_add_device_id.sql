ALTER TABLE calculations ADD COLUMN device_id TEXT NOT NULL DEFAULT 'legacy';
ALTER TABLE calculations ALTER COLUMN device_id DROP DEFAULT;
CREATE INDEX idx_calculations_device_timestamp
    ON calculations (device_id, timestamp DESC);