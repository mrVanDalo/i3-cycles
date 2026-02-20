use i3_cycles::client::send_request;
use i3_cycles::protocol::{Request, Response};

fn main() {
    match send_request(Request::Status) {
        Ok(Response::Status { json }) => {
            println!("{}", json);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
        _ => {
            eprintln!("Error: unexpected response");
            std::process::exit(1);
        }
    }
}
