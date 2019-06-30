-- Your SQL goes here
CREATE TABLE jots (
       jot_id TEXT NOT NULL PRIMARY KEY,
       jot_creation_date TEXT,
       jot_content BLOB NOT NULL,
       jot_content_type TEXT NOT NULL,
       device_id TEXT NOT NULL,
       dup_id TEXT
);

CREATE TABLE tags (
       tag_id TEXT NOT NULL PRIMARY KEY,
       tag_creation_date TEXT,
       tag_text TEXT NOT NULL,
       device_id TEXT NOT NULL,
       score INTEGER NOT NULL
);

CREATE TABLE tag_map (
       mapping_id TEXT NOT NULL PRIMARY KEY,
       tag_id TEXT NOT NULL,
       jot_id TEXT NOT NULL,
       mapping_date TEXT,
       FOREIGN KEY (tag_id) REFERENCES tags (tag_id) ON DELETE CASCADE ON UPDATE NO ACTION,
       FOREIGN KEY (jot_id) REFERENCES jots (jot_id) ON DELETE CASCADE ON UPDATE NO ACTION
);
