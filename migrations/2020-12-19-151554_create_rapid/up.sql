-- Your SQL goes here

CREATE TABLE repo (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  url TEXT NOT NULL
);

CREATE TABLE rapid_entry (
  id SERIAL PRIMARY KEY,
  repo_id INT NOT NULL,
  fullname TEXT NOT NULL,
  hash TEXT NOT NULL,
  something TEXT NOT NULL,
  alias TEXT NOT NULL,
  CONSTRAINT fk_rapid_entry
      FOREIGN KEY(repo_id)
	        REFERENCES repo(id)
);
