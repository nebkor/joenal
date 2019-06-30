use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use importer::{parse_lawg, RawJot};

fn main() {
    let mut lawg_file =
        File::open(Path::new(&env::var("HOME").unwrap()).join(".kaptanslawg")).unwrap();
    let mut lawg = String::new();
    let _ = lawg_file.read_to_string(&mut lawg);

    let jots: Vec<RawJot> = parse_lawg(lawg);

    let conn = importer::establish_connection();

    for jot in jots {
        importer::insert_jot(&conn, jot);
    }
}
