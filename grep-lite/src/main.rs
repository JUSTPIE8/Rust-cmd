use std::fs::File;
use std::io;

use std::io::{BufRead, BufReader};
fn main() {
    let sentence = "\
    hellow orld 1
    hellow orld 2
    ";

    let pattern = "helo";

    for (i, line) in sentence.lines().enumerate() {
        if sentence.contains(pattern) {
            println!("{}", line);
        }
    }
}
