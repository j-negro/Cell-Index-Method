use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_static_file() {
    let file = File::open("pepe").unwrap();

    let buf = BufReader::new(file);

    for line in buf.lines() {}
}
