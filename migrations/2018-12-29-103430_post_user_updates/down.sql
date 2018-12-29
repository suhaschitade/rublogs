-- This file should undo anything in `up.sql`

alter table posts DROP column created_at, Drop column updated_at, drop column active;

alter table users DROP column created_at, drop column updated_at, drop column active, drop column email_verify;

