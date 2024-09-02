# fullstack-crud

## How to run the project

1. Set up the `.env` by following the `.env.example` file
2. Setting up the database
    1. Spin up the database with `docker compose up`
    2. Install `sqlx-cli` with `cargo install sqlx-cli`
    3. Create the tables with `sqlx migrate run`
    4. Enable sqlx building in "offline mode" with `cargo sqlx prepare`
3. Run the project with `./scripts/run-api.sh`

## How to create tables

1. Create a migration file with `sqlx migrate add -r <migration_name>`
2. Write the up migration in the generated `.up.sql` file
3. Write the down migration in the generated `.down.sql` file
4. Run the migration with `sqlx migrate run`
5. (Optional) Rollback the migration with `sqlx migrate revert`
