CREATE TABLE users_new (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	username VARCHAR(16) UNIQUE NOT NULL,
	hashed_password VARCHAR(128) NOT NULL
);
INSERT INTO users_new(id, username, hashed_password) SELECT id, username, hashed_password FROM users;
DROP TABLE users;
ALTER TABLE users_new RENAME TO users;

CREATE TABLE projects_new (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	name VARCHAR(32) NOT NULL,
	owner_id INTEGER REFERENCES users(id) NOT NULL
);
INSERT INTO projects_new(id, name, owner_id) SELECT id, name, owner_id FROM projects;
DROP TABLE projects;
ALTER TABLE projects_new RENAME TO projects;
