use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use clap::{App, Arg, ArgMatches};

use jotlog_importer::{get_config, parse_lawg, RawJot};

const DEFAULT_LAWG: &str = "~/.kaptanslawg";
const DEFAULT_DB: &str = "~/.jotlog.sqlite";

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
    let conn = jotlog_importer::establish_connection();

    let jots: Vec<RawJot> = parse_lawg(lawg);
    for jot in jots.iter() {
        jotlog_importer::insert_jot(&conn, jot);
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
