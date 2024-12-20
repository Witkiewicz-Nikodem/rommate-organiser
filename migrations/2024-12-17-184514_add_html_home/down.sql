-- This file should undo anything in `up.sql`
DELETE FROM "HTML"
WHERE name IN ('Header', 'Footer', 'first_lines', 'logged_out_Header', 'logged_out_index_head', 'logged_out_index_body_individual', 'logged_out_support_body_individual', 'logged_out_register_body_individual', 'logged_out_form_head', 'logged_out_register_script','logged_out_login_body_individual');