use crate::daemon::get_socket_path;
use crate::protocol::{Request, Response};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

pub fn send_request(request: Request) -> Result<Response, String> {
    let socket_path = get_socket_path();

    let mut stream = UnixStream::connect(&socket_path)
        .map_err(|e| format!("Failed to connect to daemon: {}", e))?;

    let request_json = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;

    writeln!(stream, "{}", request_json).map_err(|e| format!("Failed to send request: {}", e))?;

    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    reader
        .read_line(&mut line)
        .map_err(|e| format!("Failed to read response: {}", e))?;

    let response: Response = serde_json::from_str(&line)
        .map_err(|e| format!("Failed to deserialize response: {}", e))?;

    Ok(response)
}
