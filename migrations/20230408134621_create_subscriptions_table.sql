CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY NOT NULL, 
    url TEXT NOT NULL,
    filter TEXT,
    update_interval INTEGER(4) DEFAULT 24, -- In hours
    last_updated INTEGER(4) DEFAULT (strftime('%s','now'))
);
