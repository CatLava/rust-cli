use std::env::args;

use clap::{arg, Command};


fn main() {
    println!("{:?}", std::env::args());
    let matches = Command::new("echor")
        .version("1.0")
        .author("Evan S")
        .about("simple Rust implementation of echo")
        .get_matches();
        .args();
}
