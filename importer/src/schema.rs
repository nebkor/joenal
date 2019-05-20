table! {
    jots (jot_id) {
        jot_id -> Text,
        jot_creation_date -> Nullable<Text>,
        jot_content -> Nullable<Text>,
    }
}

table! {
    tag_map (mapping_id) {
        tag_id -> Text,
        jot_id -> Text,
        mapping_date -> Nullable<Text>,
        mapping_id -> Integer,
    }
}

table! {
    tags (tag_id) {
        tag_id -> Text,
        tag_creation_date -> Nullable<Text>,
        tag_text -> Nullable<Text>,
    }
}

joinable!(tag_map -> jots (jot_id));
joinable!(tag_map -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    jots,
    tag_map,
    tags,
);
