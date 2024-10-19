imdb = imdb.db
imdb:
	docker build -t generate-imdb ./generate-db
	docker run --rm -v .:/output generate-imdb