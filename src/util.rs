use std::path::Path;

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
        let db_file = Path::new(&std::env::var("HOME").unwrap()).join(".joenal.sqlite");

        JotlogConfig {
            dev_id: dev_id.to_hyphenated().to_string(),
            db_file: db_file.to_str().unwrap().to_owned(),
        }
    }
}

pub fn get_config() -> JotlogConfig {
    confy::load("joenal").unwrap()
}

pub fn get_device_id() -> Uuid {
    let dev_id = get_config().dev_id;
    Uuid::parse_str(&dev_id).unwrap()
}

pub fn get_joenal_root() -> Uuid {
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

pub fn mk_tag_id(tag: &str) -> Uuid {
    mk_jot_ns_uuid(tag.as_bytes())
}

pub(crate) fn mk_jot_id(jot: &crate::RawJot) -> Uuid {
    let joenal_root = get_joenal_root();
    let content = [
        jot.content.as_bytes(),
        jot.creation_date.to_rfc3339().as_bytes(),
    ]
    .concat();
    Uuid::new_v5(&joenal_root, &content)
}

pub fn mk_mapping_id(jot_id: &Uuid, tag_id: &Uuid) -> Uuid {
    let data = [*jot_id.as_bytes(), *tag_id.as_bytes()].concat();
    mk_jot_ns_uuid(&data)
}
