#[macro_use]
extern crate diesel;

use chrono::prelude::*;

use lazy_static::lazy_static;
use mime::TEXT_PLAIN_UTF_8;
use regex::Regex;
use uuid::Uuid;

use std::collections::BTreeSet;

mod db;
mod models;
mod schema;
mod util;

pub use db::*;
pub use util::*;

const DSTRING: &str = "%Y-%m-%d %H:%M:%S";
pub const HOUR: i32 = 3600;

#[derive(Debug, PartialEq)]
pub struct RawJot {
    pub content: String,
    pub creation_date: DateTime<Utc>,
    pub tags: Vec<String>,
}

pub fn insert_jot(conn: &SqliteConnection, jot: &RawJot) {
    lazy_static! {
        static ref UTF_8_MIME: String = TEXT_PLAIN_UTF_8.to_string();
    }

    let mut jot_id = mk_jot_id(&jot.content.as_bytes());
    let dev_id = get_device_id();
    let creation_date = jot.creation_date.to_rfc3339();

    let mut dup_id: Option<Vec<u8>> = None;

    let jot_count: Result<i64, _> = schema::jots::table
        .filter(schema::jots::jot_id.eq(&jot_id))
        .count()
        .get_result(&*conn);

    match jot_count {
        Ok(num_rows) => {
            if num_rows > 0 {
                dup_id = Some(jot_id.clone());
                jot_id = fmt_uuid(Uuid::new_v4());
            }
        }
        _ => panic!(),
    };

    let new_jot = models::Jot::new(
        jot_id.clone(),
        Some(creation_date.clone()),
        jot.content.as_bytes().to_vec(),
        UTF_8_MIME.clone(),
        dev_id.clone(),
        dup_id,
    );

    diesel::insert_into(schema::jots::table)
        .values(&new_jot)
        .execute(&*conn)
        .unwrap_or_else(|_| panic!("couldn't insert {} into jots table.", &new_jot));

    for tag in jot.tags.iter() {
        let id = mk_tag_id(tag);
        let old_score = match schema::tags::table.find(&id).first::<models::Tag>(&*conn) {
            Ok(t) => t.get_score(),
            _ => 0,
        };

        let new_score = old_score + 1;

        let new_tag = models::Tag::new(
            tag.clone(),
            id.clone(),
            dev_id.clone(),
            Some(creation_date.clone()),
            new_score,
        );

        if old_score == 0 {
            diesel::insert_into(schema::tags::table)
                .values(&new_tag)
                .execute(&*conn)
                .unwrap();
        } else {
            diesel::update(schema::tags::table.filter(schema::tags::tag_id.eq(&id)))
                .set(schema::tags::score.eq(new_score))
                .execute(&*conn)
                .unwrap();
        }

        // now the mapping
        let mapping_id = mk_mapping_id(&jot_id, &id);

        let mapping =
            models::Mapping::new(mapping_id, id, jot_id.clone(), Some(creation_date.clone()));

        diesel::insert_into(schema::tag_map::table)
            .values(&mapping)
            .execute(&*conn)
            .expect("couldn't insert mapping");
    }
}

pub fn parse_lawg(log: String) -> Vec<RawJot> {
    lazy_static! {
        static ref TAGS: Regex = Regex::new(r"^%%TAGS%% (.*)$").unwrap();
        static ref PTZ: FixedOffset = FixedOffset::west(7 * HOUR);
        static ref DATE: Regex =
            Regex::new(r"^([0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2})").unwrap();
    }
    #[allow(non_snake_case)]
    let START = "%%START%%";
    #[allow(non_snake_case)]
    let END = "%%END%%";

    let mut jots: Vec<RawJot> = Vec::new();
    let mut content = String::new();
    let mut creation_date: DateTime<Utc> = Utc.ymd(1973, 7, 13).and_hms(0, 0, 0);
    let mut tags = vec![];

    for line in log.lines() {
        if START == line {
            continue;
        } else if DATE.captures(line).is_some() {
            creation_date = parse_date(line);
        } else if let Some(tagline) = TAGS.captures(line) {
            tags = parse_tags(&tagline[1]);
        } else if END == line {
            let jot = RawJot {
                content: content.trim().to_owned().clone(),
                creation_date,
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

pub fn parse_tags(tagline: &str) -> Vec<String> {
    let tags: BTreeSet<String> = tagline
        .split(',')
        .map(|t| t.trim().to_owned())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_lowercase())
        .collect();

    tags.into_iter().collect()
}

fn parse_date(dstring: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(dstring, DSTRING).unwrap()
}
