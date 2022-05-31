use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;
use clap::{Arg,Command};
//struct for holding arguments
//
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

fn get_args()->MyResult<Config>{

    let matches=Command::new("catr")
        .version("0.1")
        ,about("catr command using rust")
        .arg(
            Arg::new("files")

        )
}

pub fn run() -> MyResult<()> {
    println!("Hello world");
    Ok(())
}
