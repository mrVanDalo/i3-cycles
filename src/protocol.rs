use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Request {
    /// Get the next workspace in the active cycle
    Next,
    /// Toggle workspace in the active cycle
    Toggle { workspace: String },
    /// List all cycle names
    ListCycles,
    /// Select or create a cycle
    SelectCycle { name: String },
    /// Get full state for debugging
    Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Response {
    /// Next workspace to switch to, or "back_and_forth" for fallback
    NextWorkspace { workspace: String },
    /// Result of toggle operation
    Toggled { action: ToggleAction },
    /// List of cycle names
    Cycles { names: Vec<String> },
    /// Result of selecting/creating a cycle
    CycleSelected { action: SelectAction },
    /// Full state dump (JSON)
    Status { json: String },
    /// Error occurred
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ToggleAction {
    Added,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SelectAction {
    Selected,
    Created,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let requests = vec![
            Request::Next,
            Request::Toggle {
                workspace: "1".to_string(),
            },
            Request::ListCycles,
            Request::SelectCycle {
                name: "work".to_string(),
            },
            Request::Status,
        ];

        for request in requests {
            let json = serde_json::to_string(&request).unwrap();
            insta::assert_json_snapshot!(format!("{:?}", request), request);
        }
    }

    #[test]
    fn test_response_serialization() {
        let responses = vec![
            Response::NextWorkspace {
                workspace: "2".to_string(),
            },
            Response::Toggled {
                action: ToggleAction::Added,
            },
            Response::Toggled {
                action: ToggleAction::Removed,
            },
            Response::Cycles {
                names: vec!["default".to_string(), "work".to_string()],
            },
            Response::CycleSelected {
                action: SelectAction::Created,
            },
            Response::CycleSelected {
                action: SelectAction::Selected,
            },
            Response::Status {
                json: "{}".to_string(),
            },
            Response::Error {
                message: "test error".to_string(),
            },
        ];

        for response in responses {
            let json = serde_json::to_string(&response).unwrap();
            insta::assert_json_snapshot!(format!("{:?}", response), response);
        }
    }

    #[test]
    fn test_roundtrip() {
        let request = Request::Toggle {
            workspace: "test".to_string(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let deserialized: Request = serde_json::from_str(&json).unwrap();

        assert_eq!(request, deserialized);
    }
}
