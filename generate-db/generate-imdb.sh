#!/bin/bash

~/.cargo/bin/uv tool run imdb-sqlite
rm -rf ./downloads
mv ./imdb.db ./output