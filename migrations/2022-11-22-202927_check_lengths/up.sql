CREATE TABLE users_new (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	username TEXT UNIQUE NOT NULL CHECK (length(username) <= 16),
	hashed_password TEXT NOT NULL CHECK (length(hashed_password) = 128)
);
INSERT INTO users_new(id, username, hashed_password) SELECT id, username, hashed_password FROM users;
DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

CREATE TABLE projects_new (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	name TEXT NOT NULL CHECK (length(name) <= 32),
	owner_id INTEGER NOT NULL,
	FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE SET NULL
);
INSERT INTO projects_new(id, name, owner_id) SELECT id, name, owner_id FROM projects;
DROP TABLE projects;
ALTER TABLE projects_new RENAME TO projects;
