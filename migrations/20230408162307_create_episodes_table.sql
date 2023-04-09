CREATE TABLE IF NOT EXISTS episodes (
    uuid TEXT PRIMARY KEY NOT NULL, 
    url TEXT NOT NULL,
    length INTEGER(4) NOT NULL,
	type TEXT NOT NULL,
	link TEXT NOT NULL,
	image TEXT NOT NULL,
	title TEXT NOT NULL,
	description TEXT NOT NULL,
	author TEXT NOT NULL,
	duration TEXT NOT NULL,
	pub_date TEXT NOT NULL
);
	