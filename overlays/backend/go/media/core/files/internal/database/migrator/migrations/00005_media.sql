-- +goose Up
CREATE TABLE IF NOT EXISTS media_objects (
    id uuid PRIMARY KEY,
    owner_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    object_key text NOT NULL UNIQUE,
    content_type text NOT NULL,
    byte_size bigint NOT NULL CHECK (byte_size > 0),
    filename text NOT NULL,
    status text NOT NULL DEFAULT 'pending' CHECK (status IN ('pending', 'ready')),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS media_objects_owner_created_idx
    ON media_objects (owner_id, created_at DESC);

-- +goose Down
DROP TABLE IF EXISTS media_objects;
