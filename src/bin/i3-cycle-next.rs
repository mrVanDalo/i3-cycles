use i3_cycles::client::send_request;
use i3_cycles::protocol::{Request, Response};

fn main() {
    match send_request(Request::Next) {
        Ok(Response::NextWorkspace { workspace }) => {
            println!("{}", workspace);
        }
        _ => {
            // Fallback when daemon unavailable or error
            println!("back_and_forth");
        }
    }
}
