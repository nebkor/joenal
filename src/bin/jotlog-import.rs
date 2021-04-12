use std::{
    env,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};

use chrono::prelude::*;
use clap::{App, Arg, ArgMatches};
use jotlog::{get_config, parse_tags, RawJot};
use lazy_static::lazy_static;
use regex::Regex;

const DEFAULT_LAWG: &str = "~/.kaptanslawg";
const DEFAULT_DB: &str = "~/.jotlog.sqlite";

const DSTRING: &str = "%Y-%m-%d %H:%M:%S";
pub const HOUR: i32 = 3600;

fn main() {
    let args = get_args();
    let mut config = get_config();

    let lawg_file = get_lawg(&args);
    let mut lawg_file = File::open(&lawg_file).unwrap();
    let mut lawg = String::new();
    let _ = lawg_file.read_to_string(&mut lawg);

    let db_file = get_db(&args);

    config.db_file = db_file.clone();
    env::set_var("DATABASE_URL", db_file);
    let conn = jotlog::establish_connection();

    let jots: Vec<RawJot> = parse_lawg(lawg);
    for jot in jots.iter() {
        //jotlog::insert_jot(&conn, jot);
    }
}

fn get_lawg(args: &ArgMatches<'_>) -> PathBuf {
    let path = args.value_of("LAWG_FILE").unwrap();

    if path == DEFAULT_LAWG {
        Path::new(&env::var("HOME").unwrap()).join(".kaptanslawg")
    } else {
        Path::new(&path).to_path_buf()
    }
}

fn get_db(args: &ArgMatches<'_>) -> String {
    let path = args.value_of("DB_PATH").unwrap();

    let path = if path == DEFAULT_DB {
        Path::new(&env::var("HOME").unwrap()).join(".jotlog.sqlite")
    } else {
        Path::new(path).to_path_buf()
    };

    path.to_str().unwrap().to_owned()
}

fn get_args() -> ArgMatches<'static> {
    App::new("Jotlog DB importer")
        .version("1")
        .about("Imports jotlog entries from kaptanslawg flatfile into sqlite database.")
        .arg(
            Arg::with_name("LAWG_FILE")
                .help("Text file containing jotlog entries.")
                .short("l")
                .long("lawgfile")
                .default_value(DEFAULT_LAWG)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("DB_PATH")
                .help("Path where sqlite DB will be created.")
                .takes_value(true)
                .short("p")
                .long("path")
                .default_value(DEFAULT_DB),
        )
        .get_matches()
}

fn parse_lawg(log: String) -> Vec<RawJot> {
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

fn parse_date(dstring: &str) -> DateTime<Utc> {
    Utc.datetime_from_str(dstring, DSTRING).unwrap()
}
