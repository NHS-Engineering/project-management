PRAGMA foreign_keys = off;
PRAGMA legacy_alter_table = on;

ALTER TABLE users RENAME TO tmp;
CREATE TABLE users (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	username TEXT UNIQUE NOT NULL CHECK (length(username) <= 16),
	hashed_password TEXT NOT NULL CHECK (length(hashed_password) = 128)
);
INSERT INTO users(id, username, hashed_password) SELECT id, username, hashed_password FROM tmp;
DROP TABLE tmp;

ALTER TABLE projects RENAME TO tmp;
CREATE TABLE projects (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	name TEXT NOT NULL CHECK (length(name) <= 32),
	owner_id INTEGER NOT NULL,
	FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE SET NULL
);
INSERT INTO projects(id, name, owner_id) SELECT id, name, owner_id FROM tmp;
DROP TABLE tmp;
