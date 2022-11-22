PRAGMA foreign_keys = off;
PRAGMA legacy_alter_table = on;

ALTER TABLE users RENAME TO tmp;
CREATE TABLE users (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	username VARCHAR(16) UNIQUE NOT NULL,
	hashed_password VARCHAR(128) NOT NULL
);
INSERT INTO users(id, username, hashed_password) SELECT id, username, hashed_password FROM tmp;
DROP TABLE tmp;

ALTER TABLE projects RENAME TO tmp;
CREATE TABLE projects (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	name VARCHAR(32) NOT NULL,
	owner_id INTEGER REFERENCES users(id) NOT NULL
);
INSERT INTO projects(id, name, owner_id) SELECT id, name, owner_id FROM tmp;
DROP TABLE tmp;
