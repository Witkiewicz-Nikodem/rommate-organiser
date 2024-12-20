-- This file should undo anything in `up.sql`
DELETE FROM "HTML"
WHERE name IN ('logged_in_create_group_individual', 'logged_in_create_group_scripts','logged_in_create_group_head');