#[macro_use]
extern crate diesel;

use chrono::prelude::*;
use config;
use diesel::{Queryable, SqliteConnection};
use lazy_static::lazy_static;
use mime::TEXT_PLAIN_UTF_8;
use rand::prelude::*;
use regex::Regex;
use std::mem::transmute;
use std::path::Path;
use uuid::Uuid;

use std::collections::BTreeSet;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

const DSTRING: &str = "%Y-%m-%d %H:%M:%S";
const HOUR: i32 = 3600;

const NAMESPACE_JOT: &str = "930ccacb-5523-4be7-8045-f033465dae8f"; // v4 UUID used for constructing v5 UUIDs

pub mod models;
pub mod schema;

#[derive(Debug, PartialEq)]
pub struct RawJot {
    content: String,
    creation_date: DateTime<FixedOffset>,
    tags: Vec<String>,
}

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_jot(conn: &SqliteConnection, jot: &RawJot) -> usize {
    use schema::*;

    lazy_static! {
        static ref UTF_8_MIME: String = TEXT_PLAIN_UTF_8.to_string();
    }

    let dev_root = get_device_root();

    let config = get_config();
    let dev_id = config.get_str("device_id").unwrap();
    let dev_id = Uuid::parse_str(&dev_id).unwrap();

    let salt: i32 = random();
    let salt_bytes: [u8; 4] = unsafe { transmute(salt.to_le()) };
    let salted_content = [jot.content.as_bytes(), &salt_bytes].join(&0u8);
    let jot_id = gen_uuid(&dev_root, &salted_content);

    let new_post = models::Jot::new(
        jot_id.as_bytes(),
        Some(jot.creation_date.to_rfc3339()),
        jot.content.as_bytes(),
        UTF_8_MIME.as_str(),
        Some(dev_id.as_bytes()),
        salt,
    );

    diesel::insert_into(jots::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}

pub fn parse_lawg(log: String) -> Vec<RawJot> {
    lazy_static! {
        static ref START: Regex = Regex::new(r"^%%START%%$").unwrap();
        static ref END: Regex = Regex::new(r"^%%END%%").unwrap();
        static ref TAGS: Regex = Regex::new(r"^%%TAGS%% (.*)$").unwrap();
        static ref PTZ: FixedOffset = FixedOffset::west(7 * HOUR);
        static ref DATE: Regex =
            Regex::new(r"^([0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2})").unwrap();
    }

    let mut jots: Vec<RawJot> = Vec::new();
    let mut content = String::new();
    let mut creation_date: DateTime<FixedOffset> = PTZ.ymd(1973, 7, 13).and_hms(0, 0, 0);
    let mut tags = vec![];

    for line in log.lines() {
        if let Some(_) = START.captures(line) {
            continue;
        } else if let Some(_) = DATE.captures(line) {
            creation_date = parse_date(line, &PTZ);
        } else if let Some(tagline) = TAGS.captures(line) {
            tags = parse_tags(&tagline[1]);
        } else if let Some(_) = END.captures(line) {
            let jot = RawJot {
                content: content.trim().to_owned().clone(),
                creation_date: creation_date.clone(),
                tags: tags.clone(),
            };
            jots.push(jot);
            tags.clear();
            content.clear();
        } else {
            content = [&content, line].join("\n");
        }
    }

    jots
}

fn parse_tags(tagline: &str) -> Vec<String> {
    let tags: BTreeSet<String> = tagline
        .split(",")
        .map(|t| t.trim().to_owned())
        .map(|t| t.to_lowercase())
        .collect();

    tags.into_iter().collect()
}

fn parse_date(dstring: &str, tz: &FixedOffset) -> DateTime<FixedOffset> {
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

fn get_device_id() -> Uuid {
    let config = get_config();
    let dev_id = config.get_str("device_id").unwrap();
    Uuid::parse_str(&dev_id).unwrap()
}

fn get_device_root() -> Uuid {
    let dev_id = get_device_id();
    return Uuid::new_v5(&Uuid::parse_str(NAMESPACE_JOT).unwrap(), dev_id.as_bytes());
}

fn gen_uuid(root: &Uuid, content: &[u8]) -> Uuid {
    Uuid::new_v5(root, content)
}