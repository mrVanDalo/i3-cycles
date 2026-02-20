use clap::Parser;
use i3_cycles::client::send_request;
use i3_cycles::protocol::{Request, Response, SelectAction};

#[derive(Parser)]
struct Args {
    cycle_name: String,
}

fn main() {
    let args = Args::parse();

    match send_request(Request::SelectCycle {
        name: args.cycle_name,
    }) {
        Ok(Response::CycleSelected { action }) => {
            let output = match action {
                SelectAction::Created => "created",
                SelectAction::Selected => "selected",
            };
            println!("{}", output);
        }
        Err(e) => {
            eprintln!("error: {}", e);
            std::process::exit(1);
        }
        _ => {
            eprintln!("error: unexpected response");
            std::process::exit(1);
        }
    }
}
