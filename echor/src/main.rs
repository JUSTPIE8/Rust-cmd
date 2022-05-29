use clap::{Arg, Command};
fn main() {
    let matches = Command::new("echor")
        .version("0.1")
        .author("sandesh<justpie8@gmail.com>")
        .about("echo command using rust")
        .arg(
            Arg::new("text")
                .allow_invalid_utf8(true)
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::new("omit newline")
                .help("Donot print new line ")
                .takes_value(false)
                .short('n'),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
    //  println!("{:#?}", matches);
}
