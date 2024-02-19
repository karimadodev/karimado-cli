fn main() {
    std::process::exit(match karimado::cli::execute() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{:?}", err);
            1
        }
    });
}
