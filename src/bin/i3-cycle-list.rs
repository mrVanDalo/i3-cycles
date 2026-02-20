use i3_cycles::client::send_request;
use i3_cycles::protocol::{Request, Response};

fn main() {
    match send_request(Request::ListCycles) {
        Ok(Response::Cycles { names }) => {
            for name in names {
                println!("{}", name);
            }
        }
        _ => {
            // Empty output when daemon unavailable or error
        }
    }
}
