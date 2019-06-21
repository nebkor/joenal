table! {
    jots (jot_id) {
        jot_id -> Binary,
        jot_creation_date -> Nullable<Text>,
        jot_content -> Nullable<Text>,
        device_id -> Nullable<Binary>,
        salt -> Integer,
    }
}

table! {
    tag_map (mapping_id) {
        tag_id -> Binary,
        jot_id -> Binary,
        mapping_date -> Nullable<Text>,
        mapping_id -> Binary,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Binary,
        tag_creation_date -> Nullable<Text>,
        tag_text -> Text,
        device_id -> Nullable<Binary>,
    }
}

joinable!(tag_map -> jots (jot_id));
joinable!(tag_map -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    jots,
    tag_map,
    tags,
);
