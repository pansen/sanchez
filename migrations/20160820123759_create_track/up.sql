CREATE TABLE track (
    hash  TEXT PRIMARY KEY  NOT NULL,
    path  TEXT NOT NULL,
    title  TEXT NOT NULL,
    album  TEXT NOT NULL DEFAULT ""
);

CREATE UNIQUE INDEX IF NOT EXISTS track__hash on track(hash);
CREATE INDEX IF NOT EXISTS track__path on track(path);