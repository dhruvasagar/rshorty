-- Add migration script here
CREATE TABLE IF NOT EXISTS url_maps (
  key TEXT PRIMARY KEY,
  url TEXT NOT NULL
);
