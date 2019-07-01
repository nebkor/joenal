use super::Uuid;
use config;

use std::env;
use std::path::Path;

const NAMESPACE_JOT: &str = "930ccacb-5523-4be7-8045-f033465dae8f"; // v4 UUID used for constructing v5 UUIDs

pub fn get_config() -> config::Config {
    let mut config = config::Config::default();

    if let Ok(home) = env::var("HOME") {
        let conf = Path::new(&home).join(".config").join("jotlog");
        return config
            .merge(config::File::with_name(conf.to_str().unwrap()))
            .unwrap()
            .clone();
    }

    config
}

pub fn get_device_id() -> Vec<u8> {
    let config = get_config();
    let dev_id = config.get_str("device_id").unwrap();
    fmt_uuid(Uuid::parse_str(&dev_id).unwrap())
}

pub fn get_jotlog_root() -> Uuid {
    let dev_id = get_device_id();
    mk_jot_ns_uuid(&dev_id)
}

pub fn mk_jot_ns_uuid(data: &[u8]) -> Uuid {
    let jot_ns = Uuid::parse_str(NAMESPACE_JOT).unwrap();

    Uuid::new_v5(&jot_ns, data)
}

pub fn fmt_uuid(u: Uuid) -> Vec<u8> {
    u.as_bytes().to_vec()
}

pub fn mk_tag_id(tag: &str) -> Vec<u8> {
    fmt_uuid(mk_jot_ns_uuid(tag.as_bytes()))
}

pub fn mk_jot_id(content: &[u8]) -> Vec<u8> {
    let jotlog_root = get_jotlog_root();
    let jot_id = Uuid::new_v5(&jotlog_root, content);
    fmt_uuid(jot_id)
}

pub fn mk_mapping_id(jot_id: &[u8], tag_id: &[u8]) -> Vec<u8> {
    let mapping_id = mk_jot_ns_uuid(&[jot_id, tag_id].concat());
    fmt_uuid(mapping_id)
}
