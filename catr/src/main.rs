fn main() {
    let res = catr::get_args();

    if let Err(e) = catr::run(res.unwrap()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
