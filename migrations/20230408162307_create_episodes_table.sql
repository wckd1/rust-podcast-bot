CREATE TABLE IF NOT EXISTS episodes (
    uuid TEXT PRIMARY KEY NOT NULL, 
    url TEXT NOT NULL,
    length TEXT(20) NOT NULL,
	type TEXT NOT NULL,
	link TEXT NOT NULL,
	image TEXT NOT NULL,
	title TEXT NOT NULL,
	description TEXT NOT NULL,
	author TEXT NOT NULL,
	duration TEXT(20) NOT NULL,
	pub_date TEXT(20) NOT NULL
);
	