-- Add migration script here
CREATE TABLE IF NOT EXISTS log (
  id INTEGER PRIMARY KEY,
  message TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);