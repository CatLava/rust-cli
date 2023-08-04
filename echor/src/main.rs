#![feature(iter_intersperse)]
use clap::{arg, Arg, Command, value_parser, ArgAction};


fn main() {
    println!("{:?}", std::env::args());
    let matches = Command::new("echor")
        .version("1.0")
        .author("Evan S")
        .about("simple Rust implementation of echo")
        .arg(
            Arg::new("text")
            .value_parser(value_parser!(String))
            .help("input text")
            .value_name("TEXT")
            .required(true) 
            .num_args(1..)
            .action(ArgAction::Set)
        )
        .arg(
            Arg::new("omit_newline")
            .short('n') // TODO need to not allow value input
            .help("do not print new line")
        )
        .get_matches();
    let text: Vec<&String> = matches.get_many("text").expect("failed").collect();
    //println!("{:#?}", matches);
    let final_text: String = text.into_iter().cloned().intersperse(" ".to_string()).collect();
    println!("{:?}", final_text);
}
