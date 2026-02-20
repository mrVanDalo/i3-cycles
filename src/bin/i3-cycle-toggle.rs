use clap::Parser;
use i3_cycles::client::send_request;
use i3_cycles::protocol::{Request, Response, ToggleAction};

#[derive(Parser)]
struct Args {
    workspace: String,
}

fn main() {
    let args = Args::parse();

    match send_request(Request::Toggle {
        workspace: args.workspace,
    }) {
        Ok(Response::Toggled { action }) => {
            let output = match action {
                ToggleAction::Added => "added",
                ToggleAction::Removed => "removed",
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
