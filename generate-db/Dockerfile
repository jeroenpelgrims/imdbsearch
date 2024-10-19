FROM debian:stable-slim

VOLUME [ "/output" ]
RUN apt-get update && apt-get install -y curl
RUN curl -LsSf https://astral.sh/uv/install.sh -v | sh
WORKDIR /
COPY generate-imdb.sh /
RUN chmod +x /generate-imdb.sh
ENTRYPOINT ./generate-imdb.sh