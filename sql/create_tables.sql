-- Create tasks table
CREATE TABLE IF NOT EXISTS tasks (
    pubkey TEXT NOT NULL,
    daemon TEXT NOT NULL,
    status TEXT NOT NULL,
    exec_at BIGINT NOT NULL,
    PRIMARY KEY (pubkey)
);