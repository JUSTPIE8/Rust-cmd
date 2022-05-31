fn main() {
    let res = catr::get_args();

    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
