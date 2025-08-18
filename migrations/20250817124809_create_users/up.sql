CREATE TABLE IF NOT EXISTS users (
  id bigint GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
  first_name varchar(50),
  last_name varchar(50),
  ssn varchar(11)
);

