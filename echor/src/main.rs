use clap::{arg, Arg, Command};


fn main() {
    println!("{:?}", std::env::args());
    let matches = Command::new("echor")
        .version("1.0")
        .author("Evan S")
        .about("simple Rust implementation of echo")
        .arg(
            Arg::new("Text")
            .help("input text")
            .value_name("TEXT")
            .required(true)
        
        )
        .arg(
            Arg::new("omit_newline")
            .short('n') // TODO need to not allow value input
            .help("do not print new line")
        )
        .get_matches();
    println!("{:?}", matches)
}
