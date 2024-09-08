db:
	@# it runs the postgres database container
	@docker compose up

deploy:
	@# it deploys the app to shuttle without commiting the changes, but with the uncommited changes in the deployment
	@# NOTE: it needs a remote database, please change the DATABASE_URL in the .env file, and therefore local database is not needed
	@cargo shuttle deploy --ad

prepare:
	@cargo sqlx prepare

insert:
	@# it inserts the data into the database
	@cargo run --bin insert_pokemon

app:
	@# cargo watch is needed to be installed
	@# it hot reloads the app
	@cargo watch -x 'shuttle run'

tailwind:
	@# tailwindcss is needed to be installed
	@# it hot reloads the tailwindcss by watching the content files specified in tailwind.config.js
	@pnpm dlx tailwindcss -i ./assets/input.css -o ./assets/output.css --watch
