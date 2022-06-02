use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
//struct for holding arguments
//
#[derive(Debug)]
//structure of arguments
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

//using clap for getting args required for running programs
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1")
        .about("catr command using rust")
        .arg(
            Arg::new("files")
                .allow_invalid_utf8(true)
                .value_name("FILE NAME")
                .help("input file names for cat command")
                .default_value("-")
                .min_values(1),
        )
        .arg(
            Arg::new("line_number")
                .help("print line numbers space count")
                .takes_value(false)
                .short('n')
                .long("number")
                .conflicts_with("line_number_nonblank"),
        )
        .arg(
            Arg::new("line_number_nonblank")
                .help("print with line number only for non blank lines")
                .takes_value(false)
                .short('b')
                .long("number_non_blank"),
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

//opening a file if no args is provided input is taken from stdin
//BufRead is a trait with functions for reading efficiently lik readline
//bufreader  applies BufRead trait to parameters
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    /*  dbg!(&config);
    for filename in config.files {
        println!("{}", filename)
    }*/
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("failed to open {}:{}", filename, err),
            Ok(mut res) => {
                let mut last_num = 0;

                for (line_num, lines) in res.lines().enumerate() {
                    let line = lines.unwrap();

                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;

                            println!("{:>6}\t{}", last_num, line);
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}
