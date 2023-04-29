fn main() {
    if let Err(e) = raytracing::run() {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
