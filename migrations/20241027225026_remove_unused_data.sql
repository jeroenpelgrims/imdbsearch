DROP TABLE episodes;

DROP TABLE akas;

DELETE FROM
    titles
where
    type IN ('tvEpisode', "video", "videoGame", "tvPilot");

VACUUM;