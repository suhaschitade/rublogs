-- Your SQL goes here
alter table posts add column created_at date Not null, add column updated_at date Not null, add column active boolean Not null DEFAULT 'f';

alter table users add column created_at date Not null, add column updated_at date Not null, add column active boolean Not null DEFAULT 't', add column email_verify boolean Not null DEFAULT 'f';

