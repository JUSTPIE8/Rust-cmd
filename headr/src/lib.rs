use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};



#[derive(Debug)]
//structure of our cli app 

pub struct Config{
    files:Vec<String>,
    lines:usize,
    bytes:Option<usize>
}
//getting args using clap library
pub fn get_args()->MyResult<Config>{
    let matches=Command::new("headr")
        .version("0.1")
        .about("headr cli using rust")
        .arg(
            Arg::new("files")
            .allow_invalid_utf8(true)
            .value_name("File name")
            .help("input file names for headr ")
            .min_values(1)
        )
        .arg(
            Arg::new("lines")
            .help("how many lines you want to print")
            .takes_value(false)
            .default_value('10')
            .short('n')

            .long("lines")
            
        )
        .arg(
            Arg::new("bytes")
            .help("input how many bytes you want to print ")
            .takes_value(false)
            .short('b')
            .long("bytes")
        ).get_matches();

    //for parsing char to number 
let line_str=matches.value_of("lines").unwrap();
let line_num:MyResult<usize>=parse_int(line_str);
   

let bytes_str=matches.value_of("bytes").unwrap();
let bytes_num=parse_int(bytes_str);
//Returning Config struct
    Ok(Config{
        files:matches.values_of_lossy("files").unwrap(),
        lines:line_num.unwrap(),
        bytes:Some(bytes_num.unwrap())    })
}
fn parse_int(num:&str)->MyResult<usize>{
    match num.parse(){
        Ok(n) if n>0=>Ok(n),
        _=>Err(From::from(num))
    }
}

fn run(){

}
