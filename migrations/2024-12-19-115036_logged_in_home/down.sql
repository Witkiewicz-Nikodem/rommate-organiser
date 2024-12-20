-- This file should undo anything in `up.sql`
DELETE FROM "HTML"
WHERE name IN ('logged_in_basic_head','logged_in_header', 'logged_in_main', 'logged_in_home_scripts');