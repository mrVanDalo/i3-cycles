use i3_cycles::daemon;

fn main() {
    if let Err(e) = daemon::run_daemon() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
