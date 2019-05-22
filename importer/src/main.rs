use chrono::prelude::*;
use config;
use lazy_static::lazy_static;
use regex::Regex;
use std::path::Path;
use time::Timespec;
use uuid::Uuid;

use std::env;
use std::fs::File;
use std::io::prelude::*;

const DSTRING: &str = "%Y-%m-%d %H:%M:%S";
const HOUR: i32 = 3600;

const NAMESPACE_JOT: &str = "930ccacb-5523-4be7-8045-f033465dae8f"; // v4 UUID used for constructing v5 UUIDs

#[derive(Debug)]
struct Jot {
    content: String,
    creation_date: DateTime<FixedOffset>,
    tags: Vec<String>,
}

fn main() {
    let mut lawg_file =
        File::open(Path::new(&env::var("HOME").unwrap()).join(".kaptanslawg")).unwrap();
    let mut lawg = String::new();
    let _ = lawg_file.read_to_string(&mut lawg);

    let mut jots: Vec<Jot> = parse_jot(lawg);
}

fn parse_jot(log: String) -> Vec<Jot> {
    lazy_static! {
        static ref START: Regex = Regex::new(r"^%%START%%$").unwrap();
        static ref END: Regex = Regex::new(r"^%%END%%").unwrap();
        static ref TAGS: Regex = Regex::new(r"^%%TAGS%% (.*)$").unwrap();
        static ref PTZ: FixedOffset = FixedOffset::west(7 * HOUR);
    }

    let mut jots: Vec<Jot> = Vec::new();

    let mut in_jot = false;
    let mut get_date = false;

    let mut content = String::new();

    let mut creation_date: DateTime<FixedOffset> = PTZ.ymd(1973, 7, 13).and_hms(0, 0, 0);
    let mut tags = vec![];

    for line in log.lines() {
        if let Some(_) = START.captures(line) {
            in_jot = true;
            get_date = true;
            continue;
        }

        if get_date {
            creation_date = PTZ.datetime_from_str(&line, DSTRING).unwrap();
            get_date = false;
            continue;
        }

        if let Some(tagline) = TAGS.captures(line) {
            tags = parse_tags(&tagline[1]);
            continue;
        }

        if let Some(_) = END.captures(line) {
            in_jot = false;

            let jot = Jot {
                content: content.trim().to_owned().clone(),
                creation_date: creation_date.clone(),
                tags: tags.clone(),
            };

            dbg!(&jot);

            jots.push(jot);

            tags.clear();
            content.clear();
        }

        if in_jot {
            content += line;
            continue;
        }
    }

    vec![]
}

fn parse_tags(tagline: &str) -> Vec<String> {
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
    let dev_id = get_device_uuid();

    Uuid::new_v5(
        &Uuid::parse_str(NAMESPACE_JOT).unwrap(),
        &([dev_id.as_bytes(), content].concat()),
    )
}
