CREATE INDEX ix_titles_original_title ON titles (original_title);

CREATE INDEX ix_titles_primary_title ON titles (primary_title);

CREATE INDEX ix_titles_type ON titles ('type');

CREATE INDEX ix_titles_premiered ON titles (premiered);

CREATE INDEX ix_titles_runtime_minutes ON titles (runtime_minutes);

CREATE INDEX ix_titles_runtime_genres ON titles (genres);