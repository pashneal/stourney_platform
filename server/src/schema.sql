CREATE TABLE IF NOT EXISTS simplegames (
  gameuuid TEXT,
  turnid INTEGER,
  gameupdate TEXT,
  PRIMARY KEY(gameuuid, turnid)
);

CREATE TABLE IF NOT EXISTS games (
  gameuuid TEXT PRIMARY KEY,
  game_length INTEGER,
  game_over BOOLEAN
);

CREATE TABLE IF NOT EXISTS board (
  turnid INTEGER NOT NULL,
  boarduuid TEXT NOT NULL,

  deck_count_0 INTEGER NOT NULL,
  deck_count_1 INTEGER NOT NULL,
  deck_count_2 INTEGER NOT NULL,

  noble_0 INTEGER,
  noble_1 INTEGER,
  noble_2 INTEGER,
  noble_3 INTEGER,
  noble_4 INTEGER,

  onyx_count INTEGER NOT NULL,
  emerald_count INTEGER NOT NULL,
  sapphire_count INTEGER NOT NULL,
  ruby_count INTEGER NOT NULL,
  diamond_count INTEGER NOT NULL,
  gold_count INTEGER NOT NULL,

  FOREIGN KEY(boarduuid) REFERENCES games(gameuuid),
  PRIMARY KEY(boarduuid, turnid)
);

CREATE TABLE IF NOT EXISTS available_cards (
  turnid INTEGER NOT NULL,
  cardsuuid TEXT NOT NULL,

  card_0_0 INTEGER,
  card_0_1 INTEGER,
  card_0_2 INTEGER,
  card_0_3 INTEGER,

  card_1_0 INTEGER,
  card_1_1 INTEGER,
  card_1_2 INTEGER,
  card_1_3 INTEGER,

  card_2_0 INTEGER,
  card_2_1 INTEGER,
  card_2_2 INTEGER,
  card_2_3 INTEGER,

  FOREIGN KEY(cardsuuid) REFERENCES games(gameuuid),
  PRIMARY KEY(cardsuuid, turnid)
);

CREATE TABLE IF NOT EXISTS  players (
  playersuuid TEXT NOT NULL,

  turnid INTEGER NOT NULL,
  playernum INTEGER NOT NULL,

  points INTEGER NOT NULL,

  num_reserved_cards INTEGER NOT NULL,

  onyx_gem_count INTEGER NOT NULL,
  emerald_gem_count INTEGER NOT NULL,
  sapphire_gem_count INTEGER NOT NULL,
  ruby_gem_count INTEGER NOT NULL,
  diamond_gem_count INTEGER NOT NULL,
  gold_gem_count INTEGER NOT NULL,

  onyx_developments INTEGER NOT NULL,
  emerald_developments INTEGER NOT NULL,
  sapphire_developments INTEGER NOT NULL,
  ruby_developments INTEGER NOT NULL,
  diamond_developments INTEGER NOT NULL,

  FOREIGN KEY(playersuuid) REFERENCES games(gameuuid),
  PRIMARY KEY(playersuuid, turnid, playernum)
)
