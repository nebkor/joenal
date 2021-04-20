use std::path::Path;

use chrono::Utc;
use confy;
use serde::{Deserialize, Serialize};

use super::Uuid;

const NAMESPACE_JOT: &str = "930ccacb-5523-4be7-8045-f033465dae8f"; // v4 UUID used for constructing v5 UUIDs

#[derive(Serialize, Deserialize, Debug)]
pub struct JotlogConfig {
    pub dev_id: String,
    pub db_file: String,
}

impl Default for JotlogConfig {
    fn default() -> Self {
        let dev_id = Uuid::new_v4();
        let db_file = Path::new(&std::env::var("HOME").unwrap()).join(".jotlog.sqlite");

        JotlogConfig {
            dev_id: dev_id.to_hyphenated().to_string(),
            db_file: db_file.to_str().unwrap().to_owned(),
        }
    }
}

pub fn get_config() -> JotlogConfig {
    confy::load("jotlog").unwrap()
}

pub fn get_device_id() -> Vec<u8> {
    let dev_id = get_config().dev_id;
    fmt_uuid(Uuid::parse_str(&dev_id).unwrap())
}

pub fn get_jotlog_root() -> Uuid {
    let dev_id = get_config().dev_id;
    let dev_id = Uuid::parse_str(&dev_id).unwrap();
    mk_jot_ns_uuid(dev_id.as_bytes())
}

pub(crate) fn mk_jot_ns_uuid(data: &[u8]) -> Uuid {
    let jot_ns = Uuid::parse_str(NAMESPACE_JOT).unwrap();

    Uuid::new_v5(&jot_ns, data)
}

pub fn fmt_uuid(u: Uuid) -> Vec<u8> {
    u.as_bytes().to_vec()
}

pub fn mk_tag_id(tag: &str) -> Vec<u8> {
    fmt_uuid(mk_jot_ns_uuid(tag.as_bytes()))
}

pub fn mk_jot_id(jot: &crate::RawJot) -> Vec<u8> {
    let jotlog_root = get_jotlog_root();
    let content = [
        jot.content.as_bytes(),
        jot.creation_date.to_rfc3339().as_bytes(),
    ]
    .concat();
    let jot_id = Uuid::new_v5(&jotlog_root, &content);
    fmt_uuid(jot_id)
}

pub fn mk_mapping_id(jot_id: &[u8], tag_id: &[u8]) -> Vec<u8> {
    let mapping_id = mk_jot_ns_uuid(&[jot_id, tag_id].concat());
    fmt_uuid(mapping_id)
}
