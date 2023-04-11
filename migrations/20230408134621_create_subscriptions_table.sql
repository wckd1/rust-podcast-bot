CREATE TABLE IF NOT EXISTS subscriptions (
    id TEXT PRIMARY KEY NOT NULL, 
    url TEXT NOT NULL,
    filter TEXT,
    update_interval INTEGER(4) NOT NULL,
    last_updated TEXT NOT NULL
);
