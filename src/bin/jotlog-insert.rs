use std::env;
use std::io::prelude::*;
use std::io::stdin;

use clap::{App, Arg, ArgMatches};

use chrono::prelude::*;

use jotlog::{establish_connection, get_config, insert_jot, parse_tags, RawJot};

fn main() {
    let args = get_args();
    let creation_date = Utc::now();
    let content = get_content(&args);
    let tags = get_tags(&args);

    let jot = RawJot {
        content,
        creation_date,
        tags,
    };

    let config = get_config();
    env::set_var("DATABASE_URL", config.db_file);

    let conn = establish_connection();

    insert_jot(&conn, &jot);
}

fn get_args() -> ArgMatches<'static> {
    App::new("Jotlog Insert")
        .version("1")
        .about("Create and insert an entry into the jotlog database.")
        .arg(
            Arg::with_name("HEADLESS")
                .help("Do not prompt for input.")
                .long("headless")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("TAGS")
                .help("Add tag to entry; may be specified more than once for more than one tag.")
                .short("t")
                .long("tag")
                .multiple(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("MESSAGE")
                .help("Message fragment to prepend to entry.")
                .short("m")
                .long("message")
                .takes_value(true),
        )
        .get_matches()
}

fn get_content(args: &ArgMatches<'_>) -> String {
    let mut content = String::new();

    if !args.is_present("HEADLESS") {
        println!("Enter text for jotlog entry, hit ^d to end input:");
        let _ = stdin().read_to_string(&mut content);
    }

    if let Some(m) = args.value_of("MESSAGE") {
        content = [m.trim(), "\n\n", content.trim()].concat();
    }

    content.trim().to_owned()
}

fn get_tags(args: &ArgMatches<'_>) -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();

    if let Some(cli_tags) = args.values_of("TAGS") {
        let mut cli_tags = cli_tags
            .map(|t| t.trim().to_owned())
            .filter(|t| !t.is_empty())
            .collect::<Vec<_>>();
        tags.append(&mut cli_tags);
    }

    if !args.is_present("HEADLESS") {
        let mut itags = String::new();
        println!(
            "\nEnter a list of comma-separated tags (spaces allowed in tags); hit 'enter' when done."
        );
        match stdin().read_line(&mut itags) {
            Ok(_) => tags.append(&mut parse_tags(&itags)),
            _ => (),
        }
    }

    if tags.len() == 0 {
        tags.push("untagged".to_owned());
    }
    tags
}
