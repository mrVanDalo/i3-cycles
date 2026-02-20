use clap::Parser;
use i3_cycles::daemon;
use i3_cycles::logging::{self, LogFormat};

/// i3-cycle daemon - manages workspace cycles for i3wm
#[derive(Parser)]
#[command(name = "i3-cycle-daemon")]
#[command(about = "Daemon for managing i3 workspace cycles")]
struct Args {
    /// Log format: plain or json
    #[arg(long, value_name = "FORMAT", default_value = "plain")]
    log_format: String,
}

fn main() {
    let args = Args::parse();

    let log_format = match args.log_format.as_str() {
        "json" => LogFormat::Json,
        _ => LogFormat::Plain,
    };

    logging::init_logging(log_format);

    if let Err(e) = daemon::run_daemon() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
