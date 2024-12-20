-- This file should undo anything in `up.sql`
DELETE FROM "HTML"
WHERE name IN ('logged_in_belonging_groups_head','logged_in_belonging_groups_scripts');