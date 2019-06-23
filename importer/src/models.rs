use super::schema::*;
use super::{gen_uuid, Uuid, NAMESPACE_JOT};

#[derive(Queryable, Insertable)]
#[table_name = "jots"]
pub struct Jot<'j> {
    jot_id: &'j [u8],
    jot_creation_date: Option<String>,
    jot_content: &'j [u8],
    jot_content_type: &'j str,
    device_id: &'j [u8],
    salt: i32,
}

impl<'j> Jot<'j> {
    pub fn new(
        jot_id: &'j [u8],
        jot_creation_date: Option<String>,
        jot_content: &'j [u8],
        jot_content_type: &'j str,
        device_id: &'j [u8],
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

#[derive(Queryable, Insertable)]
#[table_name = "tag_map"]
pub struct Mapping<'m> {
    tag_id: &'m [u8],
    jot_id: &'m [u8],
    mapping_date: Option<String>,
    mapping_id: Vec<u8>,
}

impl<'m> Mapping<'m> {
    pub fn new(tag_id: &'m [u8], jot_id: &'m [u8], mapping_date: Option<String>) -> Self {
        let id = mk_jot_uuid(&[tag_id, jot_id].concat()).as_bytes().to_vec();

        Mapping {
            tag_id,
            jot_id,
            mapping_date,
            mapping_id: id,
        }
    }
}

#[derive(Queryable, Insertable)]
#[table_name = "tags"]
pub struct Tag<'t> {
    tag_id: Vec<u8>,
    tag_creation_date: Option<String>,
    tag_text: &'t str,
    device_id: Option<&'t [u8]>,
    score: i32,
}

impl<'t> Tag<'t> {
    pub fn new(
        tag_text: &'t str,
        device_id: Option<&'t [u8]>,
        tag_creation_date: Option<String>,
        score: i32,
    ) -> Self {
        let tag_id = mk_jot_uuid(tag_text.as_bytes()).as_bytes().to_vec();

        Tag {
            tag_id,
            tag_creation_date,
            tag_text,
            device_id,
            score,
        }
    }
}

fn mk_jot_uuid(data: &[u8]) -> Uuid {
    let jot_uuid = Uuid::parse_str(NAMESPACE_JOT).unwrap();
    gen_uuid(&jot_uuid, data)
}
