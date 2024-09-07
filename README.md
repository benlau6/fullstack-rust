# fullstack-crud

## How to run the project at the minimal setup

1. Set up the `.env` by following the `.env.example` file
2. Setting up the database
    1. Spin up the database with `docker compose up`
    2. Install `sqlx-cli` with `cargo install sqlx-cli`
    3. Create the tables with `sqlx migrate run`
    4. Enable sqlx building in "offline mode" with `cargo sqlx prepare`
3. Run the project with `cargo run`

## Technical Details of development

> Note: Check `makefile` for more runnable commands

### How to create tables

1. Create a migration file with `sqlx migrate add -r <migration_name>`
2. Write the up migration in the generated `.up.sql` file
3. Write the down migration in the generated `.down.sql` file
4. Run the migration with `sqlx migrate run`
5. (Optional) Rollback the migration with `sqlx migrate revert`

### How to live reload

It speeds up the development process by automatically recompiling the project when a file changes.

#### Live reloading the tailwindcss

1. Specify the files to be watched in `tailwind.config.js`
2. Configure the `input.css` needed by `tailwindcss` at `./assets/input.css`
3. Include the `./assets/output.css` to be compiled in the header of `base.html`
4. Run `npx tailwindcss -i ./assets/input.css -o ./assets/output.css --watch`

#### Live reloading the app

Method 1 (with browser reload) (Recommended)

1. Install `cargo-watch` with `cargo install cargo-watch`
2. Use the crate `tower-livereload` to enable live reload
3. Run the project with `cargo watch -x run`

Method 2 (with browser buffer)

1. Install `cargo-watch` with `cargo install cargo-watch`
2. Install `systemfd` with `cargo install systemfd`
3. Use the crate `listenfd` to enable buffer
4. Run the project with `systemfd --no-pid -s http::3000 -- cargo watch -x run`

[ref](https://vinnymeller.com/posts/rust-webserver-hot-reload/)

> Note: the methods are mutually exclusive

### Frontend

#### How to configure TailwindCSS

1. Install TailwindCSS with `npm install -D tailwindcss`
2. Install TailwindCSS plugins with `npm install -D @tailwindcss/forms @tailwindcss/typography @tailwindcss/aspect-ratio @tailwindcss/container-queries`

#### How to handle error response using HTMX

Normally htmx don't swap a div under [certain response status](https://htmx.org/docs/#response-handling).

1. 204 No Content by default does nothing, but is not an error
2. 2xx, 3xx and 422 responses are non-errors and are swapped
3. 4xx & 5xx responses are not swapped and are errors
4. all other responses are swapped using "..." as a catch-all

However, it can be altered by a htmx extension called [response-targets](https://github.com/bigskysoftware/htmx-extensions/blob/main/src/response-targets/README.md) to swap the div.
