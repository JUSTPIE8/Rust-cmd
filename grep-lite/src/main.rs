use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;
use clap::{Arg, Command};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

//for processing lines
fn process_lines<T: BufRead + Sized>(sentence: T, pattern: Regex) {
    for _line in sentence.lines() {
        let line = _line.unwrap();
        match pattern.find(&line) {
            Some(_) => println!("{}", line),
            None => {}
        }
    }
}

fn main() {
    //using clap for getting arguments
    let getargs = Command::new("grep-lite")
        .version("0.1")
        .about("lite version of grep")
        .arg(
            Arg::new("pattern")
                // .allow_invalid_utf8(true)
                .value_name("pattern")
                .required(true),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .takes_value(true)
                .default_value("-"),
        )
        .get_matches();

    //using regex for pattern matching
    let pattern = Regex::new(getargs.value_of("pattern").unwrap()).unwrap();
    let file = getargs.value_of("file").unwrap();
    // print!("{}", file);

    //if no file name is provided
    if file == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, pattern);
    } else {
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, pattern);
    }
}
