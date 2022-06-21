use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
//structure of our cli app

pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}
//getting args using clap library
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1")
        .about("headr cli using rust")
        .arg(
            Arg::new("files")
                .allow_invalid_utf8(true)
                .value_name("File name")
                .help("input file names for headr ")
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::new("lines")
                .help("how many lines you want to print")
                .takes_value(true)
                .default_value("10")
                .short('n')
                .long("lines"),
        )
        .arg(
            Arg::new("bytes")
                .help("input how many bytes you want to print ")
                .takes_value(true)
                .short('b')
                .long("bytes"),
        )
        .get_matches();

    //for parsing char to number
    let lines = matches
        .value_of("lines")
        .map(parse_int)
        .transpose()
        .map_err(|e| format!("illegal line count {}", e))?;

    let bytes = matches
        .value_of("bytes")
        .map(parse_int)
        .transpose()
        .map_err(|e| format!("invalid bytes count {}", e))?;
    //Returning Config struct
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes: bytes,
    })
}
//parsing str values to integer received from terminal
fn parse_int(num: &str) -> MyResult<usize> {
    match num.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(num)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}:{}", filename, err),
            Ok(_file) => println!("opened {}", filename),
        }
    }
    Ok(())
}
