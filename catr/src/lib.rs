use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;
use clap::{Arg, Command};
//struct for holding arguments
//
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1")
        .about("catr command using rust")
        .arg(
            Arg::new("files")
                .allow_invalid_utf8(true)
                .value_name("FILE NAME")
                .help("input file names for cat command")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("line_number")
                .help("print line numbers space count")
                .takes_value(false)
                .short('n'),
        )
        .arg(
            Arg::new("line_number_nonblank")
                .help("print with line number only for non blank lines")
                .takes_value(false)
                .short('b'),
        )
        .get_matches();
    let files = matches.values_of_lossy("files").unwrap();
    let line_num = matches.is_present("line_number");
    let line_num_nonblank = matches.is_present("line_number_nonblank");
    Ok(Config {
        files,
        number_lines: line_num,
        number_nonblank_lines: line_num_nonblank,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    println!("Hello world");
    Ok(())
}
