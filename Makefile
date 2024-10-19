.DEFAULT_GOAL := watch


imdb = imdb.db
imdb:
	docker build -t generate-imdb ./generate-db
	docker run --rm -v .:/output generate-imdb

css:
	bunx tailwindcss -m -i ./src/input.css -o ./assets/style.css

dependencies:
	bun install

watch: dependencies
	cargo watch -s "make css && cargo run"

release = target/release/htmxtest
release: css
	cargo build --release &&\
	cp -r assets target/release
