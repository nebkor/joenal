use super::schema::*;

#[derive(Queryable, Insertable)]
#[table_name = "jots"]
pub struct Jot<'u> {
    jot_id: &'u [u8],
    jot_creation_date: Option<String>,
    jot_content: Option<String>,
    device_id: Option<&'u [u8]>,
    salt: i32,
}

impl<'u> Jot<'u> {
    pub fn new(
        jot_id: &'u [u8],
        jot_creation_date: Option<String>,
        jot_content: Option<String>,
        device_id: Option<&'u [u8]>,
        salt: i32,
    ) -> Self {
        Jot {
            jot_id,
            jot_creation_date,
            jot_content,
            device_id,
            salt,
        }
    }
}

pub struct Tag {
    tag_id: String,
    tag_creation_date: Option<String>,
    tag_text: String,
    device_id: Option<String>,
}
