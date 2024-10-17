CREATE TABLE projects (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR(255) NOT NULL DEFAULT 'New Project',
  description VARCHAR,
  status VARCHAR(255) NOT NULL DEFAULT 'planning',
  CHECK (status IN ('planning', 'active', 'on_hold', 'blocked', 'in_review', 'completed', 'canceled'))
);

CREATE TABLE tasks (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR(255) NOT NULL DEFAULT 'New Task',
  description VARCHAR,
  status VARCHAR(255) NOT NULL DEFAULT 'todo',
  project_id INTEGER,
  FOREIGN KEY (project_id) REFERENCES projects (id),
  CHECK (status IN ('todo', 'in_progress', 'blocked', 'in_review', 'completed', 'on_hold', 'canceled'))
);
