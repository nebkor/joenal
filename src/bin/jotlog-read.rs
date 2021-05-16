use std::env;

use clap::{App, Arg, ArgMatches};
use jotlog::{get_config, get_jots, make_pool};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let _args = get_args();

    let config = get_config();
    env::set_var("DATABASE_URL", config.db_file);

    let conn = make_pool().await;

    // insert_jot(&conn, &jot);

    let jots = get_jots(&conn).await;

    for j in jots.iter() {
        println!("{}", &j);
    }

    Ok(())
}

fn get_args() -> ArgMatches<'static> {
    App::new("Jotlog Insert")
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
