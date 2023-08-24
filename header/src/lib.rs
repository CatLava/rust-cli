use clap::{arg, Arg, Command, value_parser, ArgAction};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: i32,
    bytes: Option<usize>
}

pub fn get_args() -> MyResult<Config> {
    let items = Command::new("catter")
        .version("1.0")
        .author("Evan S")
        .about("Rust implementation of cat")
        .arg(
            Arg::new("files")
                .help("list of files to cat")
                .required(true)
                .num_args(1..)
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("number_of_lines")
                .help("number the lines of catted file; bool")
                .short('n')
                .num_args(1) // used instead of takes_value
                .required(false)
        )
        .get_matches();

    Ok(Config {
        files: items.get_many("files")
            .unwrap()
            .map(|a: &String| a.to_string())
            .collect(),
        lines: items.get_one::<i32>("number_of_lines")
            .unwrap(),
        bytes: None
        })
    }

pub fn run(config: Config) -> MyResult<()> {
    println!("{:#?}", config);
    Ok(())
}
