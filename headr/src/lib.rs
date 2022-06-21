use std::error::Error;
type MyResult<T> = Result<T, Box<dyn Error>>;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
