use super::schema::*;

use std::fmt::Display;

#[derive(Queryable, Insertable, Debug)]
#[table_name = "jots"]
pub struct Jot<'j> {
    jot_id: String,
    jot_creation_date: Option<String>,
    jot_content: &'j [u8],
    jot_content_type: String,
    device_id: String,
    salt: i32,
}

impl<'j> Jot<'j> {
    pub fn new(
        jot_id: String,
        jot_creation_date: Option<String>,
        jot_content: &'j [u8],
        jot_content_type: String,
        device_id: String,
        salt: i32,
    ) -> Self {
        Jot {
            jot_id,
            jot_creation_date,
            jot_content,
            jot_content_type,
            device_id,
            salt,
        }
    }
}

impl Display for Jot<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let date = match self.jot_creation_date {
            Some(ref d) => d.clone(),
            _ => "No date".to_owned(),
        };

        write!(
            f,
            "{}\n{}\n{}",
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
            tag_id,
            jot_id,
            mapping_date,
            mapping_id,
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
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.tag_text)
    }
}
