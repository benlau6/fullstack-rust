db:
	@# it runs the postgres database container
	@docker compose up

migrate:
	@# it runs the migration to create the table in the database
	@sqlx migrate run

prepare:
	@cargo sqlx prepare

insert:
	@# it inserts the data into the database
	@cargo run --bin insert_pokemon

app:
	@# cargo watch is needed to be installed
	@# it hot reloads the app
	@cargo watch -x run

tailwind:
	@# tailwindcss is needed to be installed
	@# it hot reloads the tailwindcss by watching the content files specified in tailwind.config.js
	@pnpm dlx tailwindcss -i ./assets/input.css -o ./assets/output.css --watch
