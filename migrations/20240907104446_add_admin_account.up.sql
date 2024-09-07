-- Add up migration script here
INSERT INTO users (id, email, hashed_password, name, is_verified, is_superuser)
VALUES ('93cb6fd1-0bef-473a-87f7-655386777578', 'admin@example.com', '$2b$12$4vHMK6tEnvz5YfNQBmFPFeJqdn6gnkbB7sOhDin.eKN.4C2FzTvAC', 'Admin', true, true),
  ('00571729-af3e-4cb7-a693-ca0c82efed77', 'test@example.com', '$2b$12$4vHMK6tEnvz5YfNQBmFPFeJqdn6gnkbB7sOhDin.eKN.4C2FzTvAC', 'Alex', true, false);
