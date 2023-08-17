use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
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
                .required(true)
                .num_args(1..)
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
        )
        .arg(
            Arg::new("number_lines")
                .help("number the lines of catted file; bool")
                .short('n')
                .num_args(0) // used instead of takes_value
                .required(false)
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .help("count blank lines; bool")
                .short('b')
                .num_args(0)
                .required(false)
        )
        .get_matches();

    Ok(Config {
        files: items.get_many("files")
                        .unwrap()
                        .map(|a: &String| a.to_string())
                        .collect(),
        number_lines: items.get_flag("number_lines"),
        number_nonblank_lines: items.get_flag("number_nonblank_lines")
    })

}
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        println!("{}", filename);
        let file = match open(&filename) {
            Ok(file) => { 
                let mut lc = 0;
                for line in file.lines(){
                    let liner = line?;
                    if config.number_lines {
                        println!("{} {}", lc, liner)
                    } else {
                        println!("{}", liner);
                    }
                    lc += 1
                }
                println!("read file {}", filename);
            },
            Err(err) => eprintln!("unable to read file {}: {}", filename, err)
        };

    }
    Ok(())
}

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))), 
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}