table! {
    jots (jot_id) {
        jot_id -> Binary,
        jot_creation_date -> Nullable<Text>,
        jot_content -> Binary,
        jot_content_type -> Text,
        device_id -> Binary,
        dup_id -> Nullable<Binary>,
    }
}

table! {
    tag_map (mapping_id) {
        mapping_id -> Binary,
        tag_id -> Binary,
        jot_id -> Binary,
        mapping_date -> Nullable<Text>,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Binary,
        tag_creation_date -> Nullable<Text>,
        tag_text -> Text,
        device_id -> Binary,
        score -> Integer,
    }
}

joinable!(tag_map -> jots (jot_id));
joinable!(tag_map -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    jots,
    tag_map,
    tags,
);
