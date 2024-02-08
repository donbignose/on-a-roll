CREATE TABLE projects (
  id INTEGER not null PRIMARY KEY,
  title VARCHAR(255) NOT NULL DEFAULT 'New Project',
  description VARCHAR,
  status VARCHAR(255) NOT NULL DEFAULT 'Planning'
);

CREATE TABLE tasks (
  id INTEGER NOT NULL PRIMARY KEY,
  title VARCHAR(255) NOT NULL DEFAULT 'New Task',
  description VARCHAR,
  status VARCHAR(255) NOT NULL DEFAULT 'Todo',
  project_id INTEGER,
  FOREIGN KEY (project_id) REFERENCES projects (id)
);
