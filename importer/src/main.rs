use chrono::prelude::*;
use config;
use lazy_static::lazy_static;
use regex::Regex;
use sha2::{Digest, Sha256};
use std::path::Path;
use time::Timespec;
use uuid::Uuid;

use std::env;

const DSTRING: &str = "%Y-%m-%d %H:%M:%S";
const HOUR: i32 = 3600;

const NAMESPACE_JOT: &str = "930ccacb-5523-4be7-8045-f033465dae8f"; // v4 UUID used for constructing v5 UUIDs

struct Jot {
    content: String,
    creation_date: DateTime<FixedOffset>,
    tags: Vec<String>,
}

fn main() {
    let PTZ: FixedOffset = FixedOffset::west(7 * HOUR);

    let dev_id = get_device_uuid();

    let new_id = gen_uuid(&[]);

    let digest = make_hashed_content(&[]);

    println!(
        "dev_id: {}\nnew_id: {}\ndigest: {:?}",
        dev_id, new_id, digest
    );
}

fn parse_jot(log: String) -> Vec<Jot> {
    vec![]
}

fn parse_date(dstring: &str, tz: FixedOffset) -> DateTime<FixedOffset> {
    tz.datetime_from_str(dstring, DSTRING).unwrap()
}

fn get_config() -> config::Config {
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

fn make_hashed_content(content: &[u8]) -> String {
    format!("{:x}", sha2::Sha256::digest(content))
}

fn get_device_uuid() -> Uuid {
    let mut config = config::Config::default();

    if let Ok(home) = env::var("HOME") {
        let conf = Path::new(&home).join(".config").join("jotlog");
        let conf = config
            .merge(config::File::with_name(conf.to_str().unwrap()))
            .unwrap();
        return Uuid::parse_str(&conf.get_str("device_id").unwrap()).unwrap();
    } else {
        return Uuid::new_v4();
    }
}

fn gen_uuid(content: &[u8]) -> Uuid {
    let content = make_hashed_content(content);
    let dev_id = get_device_uuid();

    Uuid::new_v5(
        &Uuid::parse_str(NAMESPACE_JOT).unwrap(),
        &([dev_id.as_bytes(), content.as_bytes()].concat()),
    )
}
