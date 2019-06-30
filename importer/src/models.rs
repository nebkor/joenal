use super::schema::*;

use std::cmp::{Eq, PartialEq};
use std::fmt::Display;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "jots"]
pub struct Jot<'j> {
    jot_id: String,
    jot_creation_date: Option<String>,
    jot_content: &'j [u8],
    jot_content_type: String,
    device_id: String,
    dup_id: Option<String>,
}

impl<'j> Jot<'j> {
    pub fn new(
        jot_id: String,
        jot_creation_date: Option<String>,
        jot_content: &'j [u8],
        jot_content_type: String,
        device_id: String,
        dup_id: Option<String>,
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
}

impl PartialEq for Jot<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.jot_id == other.jot_id
    }
}

impl Eq for Jot<'_> {}

impl Display for Jot<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let date = match self.jot_creation_date {
            Some(ref d) => d.clone(),
            _ => "No date".to_owned(),
        };

        write!(
            f,
            "Jot: {}\nCreated: {}\n\n{}\n",
            self.jot_id,
            date,
            std::str::from_utf8(self.jot_content).unwrap()
        )
    }
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "tag_map"]
pub struct Mapping {
    mapping_id: String,
    tag_id: String,
    jot_id: String,
    mapping_date: Option<String>,
}

impl Mapping {
    pub fn new(
        mapping_id: String,
        tag_id: String,
        jot_id: String,
        mapping_date: Option<String>,
    ) -> Self {
        Mapping {
            mapping_id,
            tag_id,
            jot_id,
            mapping_date,
        }
    }
}

#[derive(Queryable, Insertable, Debug)]
#[table_name = "tags"]
pub struct Tag {
    tag_id: String,
    tag_creation_date: Option<String>,
    tag_text: String,
    device_id: String,
    score: i32,
}

impl Tag {
    pub fn new(
        tag_text: String,
        tag_id: String,
        device_id: String,
        tag_creation_date: Option<String>,
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
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.tag_text, self.score)
    }
}
