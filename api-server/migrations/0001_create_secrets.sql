CREATE TABLE IF NOT EXISTS secrets (
    id BIGINT PRIMARY KEY,
    key_name TEXT NOT NULL,
    service_name TEXT NOT NULL,
    key_value BYTEA NOT NULL,
    version INT NOT NULL DEFAULT 1 CHECK (version > 0),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    modified_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_secrets_key_name ON secrets(key_name);

CREATE UNIQUE INDEX uidx_key_name_service_name_version ON secrets(key_name, service_name, version);