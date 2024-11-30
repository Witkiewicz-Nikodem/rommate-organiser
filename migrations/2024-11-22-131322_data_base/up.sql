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
    name VARCHAR(255) UNIQUE NOT NULL,
    owner INT NOT NULL,
    FOREIGN KEY (owner) REFERENCES "user"(id) ON DELETE CASCADE
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
('Michael', 'Johnson', 'michael.johnson@example.com', 'mjohnson', 'password789'),
('Emily', 'Davis', 'emily.davis@example.com', 'emilyd', 'password321'),
('Daniel', 'Wilson', 'daniel.wilson@example.com', 'danielw', 'password654'),
('Olivia', 'Taylor', 'olivia.taylor@example.com', 'oliviat', 'password987');

-- Groups (Households)
INSERT INTO "group" (name, owner) VALUES
('Apartment A', 1),
('House B', 3),
('Studio C', 4),
('Apartment D', 2);

-- User_Group (Roommate-Household associations)
INSERT INTO "user_group" (user_id, group_id) VALUES
(1, 1), -- John and Jane in Apartment A
(2, 1), -- Jane and John in Apartment A
(3, 2), -- Michael in House B
(4, 3), -- Emily in Studio C
(5, 4), -- Daniel in Apartment D
(6, 3), -- Olivia in Studio C
(1, 4), -- John in Apartment D
(2, 2); -- Jane in House B

-- Expenses
INSERT INTO "expenses" (name, cost, user_group_id) VALUES
('Electricity Bill', 120.00, 1),  -- Expense for Apartment A
('Rent', 800.00, 1),              -- Expense for Apartment A
('Water Bill', 50.00, 2),         -- Expense for House B
('Internet Bill', 60.00, 3),      -- Expense for Studio C
('Electricity Bill', 90.00, 4),   -- Expense for Apartment D
('Rent', 950.00, 4),              -- Expense for Apartment D
('Water Bill', 30.00, 3),         -- Expense for Studio C
('Heating Bill', 40.00, 2),       -- Expense for House B
('Trash Collection', 25.00, 1),   -- Expense for Apartment A
('Internet Bill', 70.00, 2);      -- Expense for House B
