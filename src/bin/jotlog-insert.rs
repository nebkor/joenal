use std::io::prelude::*;
use std::io::stdin;

use confy;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct JotlogConfig {
    device_id: String,
    db_file: String,
}

impl Default for JotlogConfig {
    fn default() -> Self {
        JotlogConfig {
            device_id: "your mom".to_string(),
            db_file: "also your mom".to_string(),
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let cfg: JotlogConfig = confy::load("butts-jotlog")?;
    dbg!(&cfg);
    //confy::store("jotlog", &cfg)?;

    let mut input = String::new();

    stdin().read_to_string(&mut input);

    println!("Got input: {}", input);

    Ok(())
}
