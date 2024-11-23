-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,               
    first_name VARCHAR(50) NOT NULL,     
    last_name VARCHAR(50) NOT NULL,      
    email VARCHAR(255) NOT NULL UNIQUE,  
    username VARCHAR(50) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL       
);
