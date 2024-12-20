-- This file should undo anything in `up.sql`
DELETE FROM "HTML"
WHERE name IN ('logged_in_manage_groups_head','logged_in_manage_groups_individual', 'logged_in_manage_groups_scripts');