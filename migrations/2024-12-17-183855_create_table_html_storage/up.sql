-- Code to migrate the database "up"
CREATE TABLE "HTML" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    element VARCHAR(10000) NOT NULL
);
