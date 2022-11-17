ALTER TABLE tasks RENAME TO tmp;
CREATE TABLE tasks (
	id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
	name TEXT NOT NULL CHECK (length(name) <= 32),
	project_id INTEGER NOT NULL,
	assignee_id INTEGER,
	done BOOLEAN NOT NULL DEFAULT false,
	FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
	FOREIGN KEY (assignee_id) REFERENCES users(id)
);
INSERT INTO tasks(id, name, project_id, assignee_id, done) SELECT id, name, project_id, assignee_id, done FROM tmp;
DROP TABLE tmp;
