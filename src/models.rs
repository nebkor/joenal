use std::{
    cmp::{Eq, PartialEq},
    fmt::Display,
};

use sqlx::{query::Query, sqlite::SqliteArguments, FromRow, Sqlite};

use super::{StarDate, Uuid};

pub struct Content<'j> {
    pub bytes: &'j [u8],
    pub mime_type: &'j str,
}

#[derive(Clone, FromRow, Debug)]
pub struct Jot {
    jot_id: Uuid,
    jot_creation_date: Option<StarDate>,
    jot_content: Vec<u8>,
    jot_content_type: String,
    device_id: Uuid,
    dup_id: Option<Uuid>,
}

impl Jot {
    pub fn new(
        jot_id: Uuid,
        jot_creation_date: Option<StarDate>,
        jot_content: Vec<u8>,
        jot_content_type: String,
        device_id: Uuid,
        dup_id: Option<Uuid>,
    ) -> Self {
        Jot {
            jot_id,
            jot_creation_date,
            jot_content,
            jot_content_type,
            device_id,
            dup_id,
        }
    }

    pub fn as_insert(&self) -> Query<'static, Sqlite, SqliteArguments<'static>> {
        sqlx::query(r#"
INSERT INTO jots (jot_id, jot_creation_date, jot_content, jot_content_type, device_id, dup_id) VALUES (?, ?, ?, ?, ?, ?)
"#
        )
            .bind(self.jot_id)
            .bind(self.jot_creation_date)
            .bind(self.jot_content.clone())
            .bind(self.jot_content_type.clone())
            .bind(self.device_id)
            .bind(self.dup_id)
    }

    pub fn created(&self) -> Option<StarDate> {
        self.jot_creation_date
    }

    pub fn id(&self) -> Uuid {
        self.jot_id
    }

    pub fn content(&self) -> Content {
        Content {
            bytes: &self.jot_content,
            mime_type: &self.jot_content_type,
        }
    }

    pub fn button_label(&self) -> String {
        let date = if let Some(d) = self.jot_creation_date {
            d.to_rfc3339()
        } else {
            "<no date>".to_string()
        };

        let dlen = date.len();
        let date = &date[0..(10.min(dlen))];

        let content = std::str::from_utf8(&self.jot_content)
            .unwrap()
            .replace("\n", " ");
        let clen = content.len();
        let text = &content[0..(30.min(clen))];

        format!("{}: {}...", date, text)
    }
}

impl PartialEq for Jot {
    fn eq(&self, other: &Self) -> bool {
        self.jot_id == other.jot_id
    }
}

impl Eq for Jot {}

impl Display for Jot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let date = match self.jot_creation_date {
            Some(ref d) => d.to_rfc3339(),
            None => "No date".to_owned(),
        };

        write!(
            f,
            "Jot: {}\nCreated: {}\n\n{}\n",
            &self.jot_id.to_simple(),
            date,
            std::str::from_utf8(&self.jot_content).unwrap()
        )
    }
}

impl druid::Data for Jot {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

#[derive(Clone, FromRow, Debug)]
pub struct Mapping {
    mapping_id: Uuid,
    tag_id: Uuid,
    jot_id: Uuid,
    mapping_date: Option<StarDate>,
}

impl Mapping {
    pub fn new(
        mapping_id: Uuid,
        tag_id: Uuid,
        jot_id: Uuid,
        mapping_date: Option<StarDate>,
    ) -> Self {
        Mapping {
            mapping_id,
            tag_id,
            jot_id,
            mapping_date,
        }
    }

    pub fn as_insert(&self) -> Query<'static, Sqlite, SqliteArguments<'static>> {
        sqlx::query(
            r#"
INSERT INTO tag_map (mapping_id, tag_id, jot_id, mapping_date) VALUES (?, ?, ?, ?)
"#,
        )
        .bind(self.mapping_id)
        .bind(self.tag_id)
        .bind(self.jot_id)
        .bind(self.mapping_date)
    }
}

#[derive(Clone, FromRow, Debug)]
pub struct Tag {
    tag_id: Uuid,
    tag_creation_date: Option<StarDate>,
    tag_text: String,
    device_id: Uuid,
    score: i32,
}

impl Tag {
    pub fn new(
        tag_text: String,
        tag_id: Uuid,
        device_id: Uuid,
        tag_creation_date: Option<StarDate>,
        score: i32,
    ) -> Self {
        Tag {
            tag_id,
            tag_creation_date,
            tag_text,
            device_id,
            score,
        }
    }

    pub fn get_score(&self) -> i32 {
        self.score
    }

    pub fn as_insert(&self) -> Query<'static, Sqlite, SqliteArguments<'static>> {
        sqlx::query(
            r#"
INSERT INTO tags (tag_id, tag_creation_date, tag_text, device_id, score) VALUES (?, ?, ?, ?, ?)
"#,
        )
        .bind(self.tag_id)
        .bind(self.tag_creation_date)
        .bind(self.tag_text.clone())
        .bind(self.device_id)
        .bind(self.score)
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.tag_text, self.score)
    }
}
