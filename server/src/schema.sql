CREATE TABLE IF NOT EXISTS games (
  game_uuid TEXT PRIMARY KEY, 
  last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS game_updates (
  update_uuid TEXT,
  turn_id INTEGER,
  game_update TEXT,
  PRIMARY KEY(update_uuid, turn_id)
  FOREIGN KEY(update_uuid) REFERENCES games(game_uuid)
);

CREATE TABLE IF NOT EXISTS slugs (
  slug TEXT NOT NULL,
  slug_id TEXT,
  last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
  FOREIGN KEY(slug_id) REFERENCES games(game_uuid)
);

