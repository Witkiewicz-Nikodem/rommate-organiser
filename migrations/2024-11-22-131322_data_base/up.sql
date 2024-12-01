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
('Apartment E', 5),
('House F', 6),
('Condo G', 1),
('Townhouse H', 4),
('Penthouse I', 2),
('Cottage J', 3);

-- User_Group (Roommate-Household associations)
INSERT INTO "user_group" (user_id, group_id) VALUES
(1, 5), -- John in Apartment E
(2, 5), -- Jane in Apartment E
(3, 5), -- Michael in Apartment E
(4, 6), -- Emily in House F
(5, 6), -- Daniel in House F
(6, 6), -- Olivia in House F
(1, 4), -- John in Condo G
(2, 4), -- Jane in Condo G
(3, 3), -- Michael in Townhouse H
(4, 3), -- Emily in Townhouse H
(5, 3), -- Daniel in Townhouse H
(6, 2), -- Olivia in Penthouse I
(1, 2), -- John in Penthouse I
(2, 1), -- Jane in Cottage J
(3, 1), -- Michael in Cottage J
(4, 1); -- Emily in Cottage J

-- Expenses
INSERT INTO "expenses" (name, cost, user_group_id) VALUES
('Electricity Bill', 130.00, 1),  -- Expense for Apartment A
('Rent', 850.00, 1),              -- Expense for Apartment A
('Water Bill', 55.00, 2),         -- Expense for House B
('Internet Bill', 65.00, 3),      -- Expense for Studio C
('Electricity Bill', 100.00, 4),  -- Expense for Apartment D
('Rent', 960.00, 4),              -- Expense for Apartment D
('Water Bill', 35.00, 3),         -- Expense for Studio C
('Heating Bill', 45.00, 2),       -- Expense for House B
('Trash Collection', 30.00, 1),   -- Expense for Apartment A
('Internet Bill', 80.00, 2),      -- Expense for House B
('Electricity Bill', 140.00, 5),  -- Expense for Apartment E
('Rent', 900.00, 5),              -- Expense for Apartment E
('Water Bill', 60.00, 6),         -- Expense for House F
('Internet Bill', 75.00, 6),      -- Expense for House F
('Heating Bill', 50.00, 6),       -- Expense for House F
('Trash Collection', 40.00, 6),   -- Expense for House F
('Rent', 1100.00, 7),             -- Expense for Condo G
('Electricity Bill', 150.00, 7),  -- Expense for Condo G
('Internet Bill', 85.00, 8),      -- Expense for Townhouse H
('Water Bill', 45.00, 8),         -- Expense for Townhouse H
('Rent', 950.00, 8),              -- Expense for Townhouse H
('Electricity Bill', 120.00, 9),  -- Expense for Penthouse I
('Trash Collection', 35.00, 9),   -- Expense for Penthouse I
('Rent', 1500.00, 9),             -- Expense for Penthouse I
('Internet Bill', 100.00, 10),    -- Expense for Cottage J
('Electricity Bill', 110.00, 10), -- Expense for Cottage J
('Water Bill', 40.00, 10),        -- Expense for Cottage J
('Electricity Bill', 150.00, 11),  -- Expense for group 11
('Rent', 950.00, 11),              -- Expense for group 11
('Water Bill', 40.00, 11),         -- Expense for group 11
('Internet Bill', 60.00, 11),      -- Expense for group 11
('Electricity Bill', 130.00, 12),  -- Expense for group 12
('Rent', 1200.00, 12),             -- Expense for group 12
('Water Bill', 50.00, 12),         -- Expense for group 12
('Internet Bill', 80.00, 12),      -- Expense for group 12
('Electricity Bill', 140.00, 13),  -- Expense for group 13
('Rent', 1100.00, 13),             -- Expense for group 13
('Water Bill', 45.00, 13),         -- Expense for group 13
('Heating Bill', 55.00, 13),       -- Expense for group 13
('Electricity Bill', 120.00, 14),  -- Expense for group 14
('Rent', 900.00, 14),              -- Expense for group 14
('Water Bill', 35.00, 14),         -- Expense for group 14
('Trash Collection', 25.00, 14);   -- Expense for group 14