-- Code to migrate the database "up"
CREATE TABLE "user" (
    id SERIAL PRIMARY KEY,
    first_name VARCHAR(255) NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL
);

CREATE TABLE "group" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL
);

CREATE TABLE "user_group" (
    user_group_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    group_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES "group"(id) ON DELETE CASCADE
);

CREATE TABLE "expenses" (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    cost NUMERIC(10, 2) NOT NULL,
    user_group_id INT NOT NULL,
    FOREIGN KEY (user_group_id) REFERENCES "user_group"(user_group_id) ON DELETE CASCADE
);

-- Insert sample data
-- Users (Roommates)
INSERT INTO "user" (first_name, last_name, email, username, password) VALUES
('John', 'Doe', 'john.doe@example.com', 'johndoe', 'password123'),
('Jane', 'Smith', 'jane.smith@example.com', 'janesmith', 'password456'),
('Michael', 'Johnson', 'michael.johnson@example.com', 'mjohnson', 'password789');

-- Groups (Households)
INSERT INTO "group" (name) VALUES
('Apartment A'),
('House B'),
('Shared Dorm');

-- User_Group (Roommate-Household associations)
INSERT INTO "user_group" (user_id, group_id) VALUES
(1, 1), -- John and Jane in Apartment A
(2, 1), -- Jane and John in Apartment A
(3, 2); -- Michael in House B

-- Expenses
INSERT INTO "expenses" (name, cost, user_group_id) VALUES
('Electricity Bill', 120.00, 1),  -- Expense for Apartment A
('Rent', 800.00, 1),              -- Expense for Apartment A
('Water Bill', 50.00, 2);         -- Expense for House B