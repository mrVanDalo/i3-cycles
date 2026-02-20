use crate::protocol::{Request, Response, SelectAction, ToggleAction};
use crate::state::Cycles;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

fn socket_path() -> PathBuf {
    let runtime_dir = std::env::var("XDG_RUNTIME_DIR")
        .unwrap_or_else(|_| "/tmp".to_string());
    PathBuf::from(runtime_dir).join("i3-cycles.sock")
}

pub fn run_daemon() -> Result<(), String> {
    let socket_path = socket_path();

    // Remove existing socket if present
    if socket_path.exists() {
        std::fs::remove_file(&socket_path)
            .map_err(|e| format!("Failed to remove existing socket: {}", e))?;
    }

    let listener = UnixListener::bind(&socket_path)
        .map_err(|e| format!("Failed to bind socket: {}", e))?;

    eprintln!("i3-cycle-daemon listening on {:?}", socket_path);

    let mut cycles = Cycles::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream, &mut cycles) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Connection error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: UnixStream, cycles: &mut Cycles) -> Result<(), String> {
    let mut reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
    let mut line = String::new();

    reader.read_line(&mut line).map_err(|e| e.to_string())?;

    let request: Request = serde_json::from_str(&line)
        .map_err(|e| format!("Invalid request: {}", e))?;

    let response = handle_request(request, cycles);

    let response_json = serde_json::to_string(&response)
        .map_err(|e| format!("Failed to serialize response: {}", e))?;

    writeln!(stream, "{}", response_json).map_err(|e| e.to_string())?;

    Ok(())
}

fn handle_request(request: Request, cycles: &mut Cycles) -> Response {
    match request {
        Request::Next => {
            let workspace = cycles.next()
                .unwrap_or_else(|| "back_and_forth".to_string());
            Response::NextWorkspace { workspace }
        }
        Request::Toggle { workspace } => {
            let added = cycles.toggle(workspace);
            let action = if added {
                ToggleAction::Added
            } else {
                ToggleAction::Removed
            };
            Response::Toggled { action }
        }
        Request::ListCycles => {
            let names = cycles.list_cycles();
            Response::Cycles { names }
        }
        Request::SelectCycle { name } => {
            let created = cycles.select_cycle(name);
            let action = if created {
                SelectAction::Created
            } else {
                SelectAction::Selected
            };
            Response::CycleSelected { action }
        }
        Request::Status => {
            let json = cycles.to_json();
            Response::Status { json }
        }
    }
}

pub fn get_socket_path() -> PathBuf {
    socket_path()
}
