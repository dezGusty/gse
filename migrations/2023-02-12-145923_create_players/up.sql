-- Your SQL goes here

CREATE TABLE "players" (
	"id"	INTEGER UNIQUE,
	"name"	TEXT NOT NULL,
	"display_name"	TEXT,
	"rating"	REAL NOT NULL,
	PRIMARY KEY("id")
);