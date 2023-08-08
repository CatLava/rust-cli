use std::error::Error;
use clap::{arg, Arg, Command, value_parser, ArgAction};


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}
type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    arg = Command::new("catter")
        .version("1.0")
        .author("Evan S")
        .about("Rust implementation of cat")
        .arg(
            .Arg::new("files")
                .help("list of files to cat")
                .required(true)
                .num_args(1..)
                .action(ArgAction::Set)
        )
        .arg(
            .Arg::new("number_lines")
                .help("number the lines of catted file; bool")
                .short("-nl")
                .required(true)
        )
        .arg(
            .Arg::new("number_nonblank_lines")
                .help("count blank lines; bool")
                .short("-b")
                .required(true)
        )

}
pub fn run() -> MyResult<()> {
    println!("Hello");
    Ok(())
}