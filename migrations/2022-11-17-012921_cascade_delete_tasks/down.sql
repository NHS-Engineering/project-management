ALTER TABLE tasks RENAME TO tmp;
CREATE TABLE tasks (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	name VARCHAR(32) NOT NULL,
	project_id INTEGER REFERENCES projects(id) NOT NULL,
	assignee_id INTEGER REFERENCES users(id),
	done BOOLEAN NOT NULL DEFAULT false
);
INSERT INTO tasks(id, name, project_id, assignee_id, done) SELECT id, name, project_id, assignee_id, done FROM tmp;
DROP TABLE tmp;
