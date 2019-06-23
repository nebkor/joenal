use super::schema::*;

#[derive(Queryable, Insertable)]
#[table_name = "jots"]
pub struct Jot<'r> {
    // 'r is the lifetime of the RawJot used to build this Jot
    jot_id: &'r [u8],
    jot_creation_date: Option<String>,
    jot_content: &'r [u8],
    jot_content_type: &'r str,
    device_id: Option<&'r [u8]>,
    salt: i32,
}

impl<'r> Jot<'r> {
    pub fn new(
        jot_id: &'r [u8],
        jot_creation_date: Option<String>,
        jot_content: &'r [u8],
        jot_content_type: &'r str,
        device_id: Option<&'r [u8]>,
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

pub struct Tag<'r> {
    tag_id: &'r [u8],
    tag_creation_date: Option<String>,
    tag_text: String,
    device_id: Option<&'r [u8]>,
    score: i32,
}
