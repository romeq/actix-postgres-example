-- Your SQL goes here
CREATE TABLE files(
    file_id             UUID            PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id            UUID            NOT NULL,
    original_filename   VARCHAR(256)    NOT NULL, 
    was_encrypted       BOOLEAN         NOT NULL DEFAULT FALSE,
    uploaded_at         TIMESTAMP       NOT NULL DEFAULT CURRENT_TIMESTAMP,
    encryption_iv       VARCHAR(256),

    FOREIGN KEY (owner_id) REFERENCES users(user_id)
);
