fn main() {
    if let Err(e) = catter::run() {
        eprintln!("{}", e); 
        std::process::exit(1);
    }
}
