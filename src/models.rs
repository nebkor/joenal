use std::{
    cmp::{Eq, PartialEq},
    fmt::Display,
};

use sqlx::prelude::*;

use super::Uuid;

#[derive(FromRow, Debug)]
pub struct Jot {
    jot_id: Vec<u8>,
    jot_creation_date: Option<String>,
    jot_content: Vec<u8>,
    jot_content_type: String,
    device_id: Vec<u8>,
    dup_id: Option<Vec<u8>>,
}

impl Jot {
    pub fn new(
        jot_id: Vec<u8>,
        jot_creation_date: Option<String>,
        jot_content: Vec<u8>,
        jot_content_type: String,
        device_id: Vec<u8>,
        dup_id: Option<Vec<u8>>,
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

impl PartialEq for Jot {
    fn eq(&self, other: &Self) -> bool {
        self.jot_id == other.jot_id
    }
}

impl Eq for Jot {}

impl Display for Jot {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let date = match self.jot_creation_date {
            Some(ref d) => d.clone(),
            None => "No date".to_owned(),
        };

        write!(
            f,
            "Jot: {}\nCreated: {}\n\n{}\n",
            Uuid::from_slice(&self.jot_id).unwrap().to_simple(),
            date,
            std::str::from_utf8(&self.jot_content).unwrap()
        )
    }
}

#[derive(FromRow, Debug)]
pub struct Mapping {
    mapping_id: Vec<u8>,
    tag_id: Vec<u8>,
    jot_id: Vec<u8>,
    mapping_date: Option<String>,
}

impl Mapping {
    pub fn new(
        mapping_id: Vec<u8>,
        tag_id: Vec<u8>,
        jot_id: Vec<u8>,
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

#[derive(FromRow, Debug)]
pub struct Tag {
    tag_id: Vec<u8>,
    tag_creation_date: Option<String>,
    tag_text: String,
    device_id: Vec<u8>,
    score: i32,
}

impl Tag {
    pub fn new(
        tag_text: String,
        tag_id: Vec<u8>,
        device_id: Vec<u8>,
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
