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
    let items = Command::new("catter")
        .version("1.0")
        .author("Evan S")
        .about("Rust implementation of cat")
        .arg(
            Arg::new("files")
                .help("list of files to cat")
                .short('f')
                .required(true)
                .num_args(1..)
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("number_lines")
                .help("number the lines of catted file; bool")
                .short('n')
                .required(false)
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .help("count blank lines; bool")
                .short('b')
                .required(false)
        )
        .get_matches();

    Ok(Config {
        files: items.get_many("files")
                        .unwrap()
                        .map(|a: &String| a.to_string())
                        .collect(),
        number_lines: false,
        number_nonblank_lines: false
    })

}
pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}