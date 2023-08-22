use clap::{arg, Arg, Command, value_parser, ArgAction};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct Config {
    files: Vec<String>,
    lines: i32,
    bytes: Option<usize>
}