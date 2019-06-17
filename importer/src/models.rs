struct Jot {
    jot_id: String,
    jot_creation_date: Option<String>,
    jot_content: Option<String>,
    device_id: Option<String>,
}

struct Tag {
    tag_id: String,
    tag_creation_date: Option<String>,
    tag_text: String,
    device_id: Option<String>,
}
