fn main() {
    if let Err(e) = catter::get_args()
                            .and_then(catter::run) {
        eprintln!("{}", e); 
        std::process::exit(1);
    }
}
