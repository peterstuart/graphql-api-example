CREATE TYPE status AS ENUM ('incomplete', 'complete');

CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  status status NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users (id)
)

