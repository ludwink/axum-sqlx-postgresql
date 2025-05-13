-- Add up migration script here
CREATE TABLE
  IF NOT EXISTS authors (
    id SERIAL,
    name VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    date_of_birth DATE,
    CONSTRAINT pk_authors PRIMARY KEY (id)
  );

INSERT INTO
  authors (name, email, date_of_birth)
VALUES
  (
    'Mario Vargas Llosa',
    'llosa@email.com',
    '1936-03-28'
  ),
  (
    'Jorge Luis Borges',
    'borges@email.com',
    '1899-08-24'
  );
