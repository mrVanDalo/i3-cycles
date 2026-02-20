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

    // Individual Request variant serialization tests

    #[test]
    fn test_request_next_serialization() {
        let request = Request::Next;
        insta::assert_json_snapshot!(request);
    }

    #[test]
    fn test_request_toggle_serialization() {
        // Test with simple workspace name
        let request = Request::Toggle {
            workspace: "1".to_string(),
        };
        insta::assert_json_snapshot!("toggle_workspace_1", request);

        // Test with named workspace
        let request = Request::Toggle {
            workspace: "browser".to_string(),
        };
        insta::assert_json_snapshot!("toggle_workspace_browser", request);

        // Test with workspace containing special characters
        let request = Request::Toggle {
            workspace: "workspace-2_test".to_string(),
        };
        insta::assert_json_snapshot!("toggle_workspace_special", request);
    }

    #[test]
    fn test_request_list_cycles_serialization() {
        let request = Request::ListCycles;
        insta::assert_json_snapshot!(request);
    }

    #[test]
    fn test_request_select_cycle_serialization() {
        // Test with default cycle name
        let request = Request::SelectCycle {
            name: "default".to_string(),
        };
        insta::assert_json_snapshot!("select_cycle_default", request);

        // Test with custom cycle name
        let request = Request::SelectCycle {
            name: "work".to_string(),
        };
        insta::assert_json_snapshot!("select_cycle_work", request);

        // Test with cycle name containing special characters
        let request = Request::SelectCycle {
            name: "my-cycle_123".to_string(),
        };
        insta::assert_json_snapshot!("select_cycle_special", request);
    }

    #[test]
    fn test_request_status_serialization() {
        let request = Request::Status;
        insta::assert_json_snapshot!(request);
    }

    #[test]
    fn test_request_deserialization() {
        // Test Next deserialization
        let json = r#""Next""#;
        let deserialized: Request = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, Request::Next);

        // Test Toggle deserialization
        let json = r#"{"Toggle":{"workspace":"1"}}"#;
        let deserialized: Request = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Request::Toggle {
                workspace: "1".to_string()
            }
        );

        // Test ListCycles deserialization
        let json = r#""ListCycles""#;
        let deserialized: Request = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, Request::ListCycles);

        // Test SelectCycle deserialization
        let json = r#"{"SelectCycle":{"name":"work"}}"#;
        let deserialized: Request = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Request::SelectCycle {
                name: "work".to_string()
            }
        );

        // Test Status deserialization
        let json = r#""Status""#;
        let deserialized: Request = serde_json::from_str(json).unwrap();
        assert_eq!(deserialized, Request::Status);
    }

    #[test]
    fn test_request_roundtrip_variants() {
        // Test roundtrip for all Request variants
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

        for original in requests {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Request = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Roundtrip failed for {:?}",
                original
            );
        }
    }

    // Individual Response variant serialization tests

    #[test]
    fn test_response_next_workspace() {
        // Test with simple workspace name
        let response = Response::NextWorkspace {
            workspace: "2".to_string(),
        };
        insta::assert_json_snapshot!("next_workspace_2", response);

        // Test with named workspace
        let response = Response::NextWorkspace {
            workspace: "browser".to_string(),
        };
        insta::assert_json_snapshot!("next_workspace_browser", response);

        // Test with special characters
        let response = Response::NextWorkspace {
            workspace: "workspace-3_test".to_string(),
        };
        insta::assert_json_snapshot!("next_workspace_special", response);
    }

    #[test]
    fn test_response_toggled_added() {
        let response = Response::Toggled {
            action: ToggleAction::Added,
        };
        insta::assert_json_snapshot!(response);
    }

    #[test]
    fn test_response_toggled_removed() {
        let response = Response::Toggled {
            action: ToggleAction::Removed,
        };
        insta::assert_json_snapshot!(response);
    }

    #[test]
    fn test_response_cycles() {
        // Test with empty list
        let response = Response::Cycles { names: vec![] };
        insta::assert_json_snapshot!("cycles_empty", response);

        // Test with single cycle
        let response = Response::Cycles {
            names: vec!["default".to_string()],
        };
        insta::assert_json_snapshot!("cycles_single", response);

        // Test with multiple cycles
        let response = Response::Cycles {
            names: vec![
                "default".to_string(),
                "work".to_string(),
                "personal".to_string(),
            ],
        };
        insta::assert_json_snapshot!("cycles_multiple", response);
    }

    #[test]
    fn test_response_cycle_selected_created() {
        let response = Response::CycleSelected {
            action: SelectAction::Created,
        };
        insta::assert_json_snapshot!(response);
    }

    #[test]
    fn test_response_cycle_selected_selected() {
        let response = Response::CycleSelected {
            action: SelectAction::Selected,
        };
        insta::assert_json_snapshot!(response);
    }

    #[test]
    fn test_response_status() {
        // Test with empty JSON
        let response = Response::Status {
            json: "{}".to_string(),
        };
        insta::assert_json_snapshot!("status_empty", response);

        // Test with simple JSON
        let response = Response::Status {
            json: r#"{"active_cycle":"default","cycles":[]}"#.to_string(),
        };
        insta::assert_json_snapshot!("status_simple", response);

        // Test with complex JSON
        let response = Response::Status {
            json: r#"{"active_cycle":"work","cycles":[{"name":"work","workspaces":["1","2"],"position":0}]}"#.to_string(),
        };
        insta::assert_json_snapshot!("status_complex", response);
    }

    #[test]
    fn test_response_error() {
        // Test with simple error
        let response = Response::Error {
            message: "test error".to_string(),
        };
        insta::assert_json_snapshot!("error_simple", response);

        // Test with error containing special characters
        let response = Response::Error {
            message: "Error: workspace 'test' not found!".to_string(),
        };
        insta::assert_json_snapshot!("error_special", response);

        // Test with multi-line error
        let response = Response::Error {
            message: "Multiple errors:\n- Line 1\n- Line 2".to_string(),
        };
        insta::assert_json_snapshot!("error_multiline", response);
    }

    #[test]
    fn test_response_deserialization() {
        // Test NextWorkspace deserialization
        let json = r#"{"NextWorkspace":{"workspace":"2"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::NextWorkspace {
                workspace: "2".to_string()
            }
        );

        // Test Toggled with Added deserialization
        let json = r#"{"Toggled":{"action":"Added"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::Toggled {
                action: ToggleAction::Added
            }
        );

        // Test Toggled with Removed deserialization
        let json = r#"{"Toggled":{"action":"Removed"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::Toggled {
                action: ToggleAction::Removed
            }
        );

        // Test Cycles deserialization
        let json = r#"{"Cycles":{"names":["default","work"]}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::Cycles {
                names: vec!["default".to_string(), "work".to_string()]
            }
        );

        // Test CycleSelected with Created deserialization
        let json = r#"{"CycleSelected":{"action":"Created"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::CycleSelected {
                action: SelectAction::Created
            }
        );

        // Test CycleSelected with Selected deserialization
        let json = r#"{"CycleSelected":{"action":"Selected"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::CycleSelected {
                action: SelectAction::Selected
            }
        );

        // Test Status deserialization
        let json = r#"{"Status":{"json":"{}"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::Status {
                json: "{}".to_string()
            }
        );

        // Test Error deserialization
        let json = r#"{"Error":{"message":"test error"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::Error {
                message: "test error".to_string()
            }
        );
    }

    #[test]
    fn test_response_roundtrip_variants() {
        // Test roundtrip for all Response variants
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

        for original in responses {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Response = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Roundtrip failed for {:?}",
                original
            );
        }
    }

    // Error handling and malformed request tests (PROT-03, PROT-04)

    #[test]
    fn test_error_response_variants() {
        // Test with empty message
        let response = Response::Error {
            message: "".to_string(),
        };
        insta::assert_json_snapshot!("error_empty_message", response);

        // Test with short message
        let response = Response::Error {
            message: "Oops".to_string(),
        };
        insta::assert_json_snapshot!("error_short_message", response);

        // Test with long message
        let response = Response::Error {
            message: "This is a very long error message that explains in detail what went wrong and provides guidance on how to fix the issue".to_string(),
        };
        insta::assert_json_snapshot!("error_long_message", response);

        // Test with special characters
        let response = Response::Error {
            message: "Error: file 'test.txt' not found! (code: 404)".to_string(),
        };
        insta::assert_json_snapshot!("error_special_chars", response);

        // Test with unicode
        let response = Response::Error {
            message: "Error: invalid UTF-8 character encountered".to_string(),
        };
        insta::assert_json_snapshot!("error_unicode", response);
    }

    #[test]
    fn test_malformed_json_handling() {
        // Test invalid JSON syntax
        let result: Result<Request, _> = serde_json::from_str("{invalid");
        assert!(result.is_err());
        insta::assert_snapshot!("malformed_invalid_json", result.unwrap_err().to_string());

        // Test non-JSON text
        let result: Result<Request, _> = serde_json::from_str("not json at all");
        assert!(result.is_err());
        insta::assert_snapshot!("malformed_not_json", result.unwrap_err().to_string());

        // Test empty JSON object (should be an error - no valid variant)
        let result: Result<Request, _> = serde_json::from_str("{}");
        assert!(result.is_err());
        insta::assert_snapshot!("malformed_empty_object", result.unwrap_err().to_string());

        // Test null (should be an error - no valid variant)
        let result: Result<Request, _> = serde_json::from_str("null");
        assert!(result.is_err());
        insta::assert_snapshot!("malformed_null", result.unwrap_err().to_string());

        // Test truncated JSON
        let result: Result<Request, _> = serde_json::from_str(r#"{"Toggle": {"workspace": "1""#);
        assert!(result.is_err());
        insta::assert_snapshot!("malformed_truncated", result.unwrap_err().to_string());

        // Test invalid syntax with extra commas
        let result: Result<Request, _> = serde_json::from_str(r#"{"Toggle": {"workspace": "1",}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("malformed_trailing_comma", result.unwrap_err().to_string());

        // Test unquoted keys
        let result: Result<Request, _> = serde_json::from_str(r#"{Toggle: {"workspace": "1"}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("malformed_unquoted_key", result.unwrap_err().to_string());

        // Same tests for Response
        let result: Result<Response, _> = serde_json::from_str("{invalid response");
        assert!(result.is_err());
    }

    #[test]
    fn test_missing_field_handling() {
        // Toggle without workspace field - serde will deserialize but field will be missing
        let result: Result<Request, _> = serde_json::from_str(r#"{"Toggle": {}}"#);
        // This will fail because workspace is a required String field
        assert!(result.is_err());
        insta::assert_snapshot!("missing_field_toggle", result.unwrap_err().to_string());

        // SelectCycle without name field
        let result: Result<Request, _> = serde_json::from_str(r#"{"SelectCycle": {}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!(
            "missing_field_select_cycle",
            result.unwrap_err().to_string()
        );

        // NextWorkspace without workspace field
        let result: Result<Response, _> = serde_json::from_str(r#"{"NextWorkspace": {}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!(
            "missing_field_next_workspace",
            result.unwrap_err().to_string()
        );

        // Error without message field
        let result: Result<Response, _> = serde_json::from_str(r#"{"Error": {}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("missing_field_error", result.unwrap_err().to_string());
    }

    #[test]
    fn test_unknown_variant_handling() {
        // Unknown Request variant
        let result: Result<Request, _> = serde_json::from_str(r#"{"UnknownVariant": {}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("unknown_variant_request", result.unwrap_err().to_string());

        // Unknown Response variant
        let result: Result<Response, _> = serde_json::from_str(r#"{"UnknownResponse": {}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("unknown_variant_response", result.unwrap_err().to_string());

        // Unknown variant with valid structure
        let result: Result<Request, _> = serde_json::from_str(r#"{"Unknown": {"workspace": "1"}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!(
            "unknown_variant_with_fields",
            result.unwrap_err().to_string()
        );

        // Misspelled variant name
        let result: Result<Request, _> = serde_json::from_str(r#"{"Nxt": {}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("unknown_variant_typo", result.unwrap_err().to_string());

        // Case sensitivity test
        let result: Result<Request, _> = serde_json::from_str(r#""next""#);
        assert!(result.is_err());
        insta::assert_snapshot!("unknown_variant_lowercase", result.unwrap_err().to_string());
    }

    #[test]
    fn test_error_response_deserialization() {
        // Deserialize simple error
        let json = r#"{"Error":{"message":"Something went wrong"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::Error {
                message: "Something went wrong".to_string()
            }
        );

        // Deserialize error with special characters
        let json = r#"{"Error":{"message":"Error: 'file' not found!"}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::Error {
                message: "Error: 'file' not found!".to_string()
            }
        );

        // Deserialize empty error message
        let json = r#"{"Error":{"message":""}}"#;
        let deserialized: Response = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            Response::Error {
                message: "".to_string()
            }
        );

        // Test roundtrip for error responses
        let error_responses = vec![
            Response::Error {
                message: "Error 1".to_string(),
            },
            Response::Error {
                message: "Error with spaces and symbols! @#$%".to_string(),
            },
            Response::Error {
                message: "".to_string(),
            },
        ];

        for original in error_responses {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: Response = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Roundtrip failed for error response"
            );
        }
    }

    #[test]
    fn test_complex_error_scenarios() {
        // Wrong type for workspace field (number instead of string)
        let result: Result<Request, _> = serde_json::from_str(r#"{"Toggle": {"workspace": 123}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("error_wrong_type_number", result.unwrap_err().to_string());

        // Wrong type - boolean instead of string
        let result: Result<Request, _> = serde_json::from_str(r#"{"Toggle": {"workspace": true}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("error_wrong_type_boolean", result.unwrap_err().to_string());

        // Wrong type - null instead of string
        let result: Result<Request, _> = serde_json::from_str(r#"{"Toggle": {"workspace": null}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!("error_wrong_type_null", result.unwrap_err().to_string());

        // Note: serde rejects unknown fields in externally tagged enums, so extra fields cause errors
        // This is correct behavior - protocol is strict about field names
        let result: Result<Request, _> =
            serde_json::from_str(r#"{"Next": {}, "extra": "rejected"}"#);
        assert!(result.is_err());
        insta::assert_snapshot!(
            "error_extra_field_rejected",
            result.unwrap_err().to_string()
        );

        // Empty string in required field (this is valid - empty string is still a string)
        let result: Result<Request, _> = serde_json::from_str(r#"{"Toggle": {"workspace": ""}}"#);
        assert!(result.is_ok());
        let request = result.unwrap();
        assert_eq!(
            request,
            Request::Toggle {
                workspace: "".to_string()
            }
        );
        insta::assert_snapshot!(
            "error_empty_string_valid",
            "Empty string is valid for workspace field"
        );

        // Deeply nested invalid structure
        let result: Result<Request, _> =
            serde_json::from_str(r#"{"Toggle": {"workspace": {"nested": "invalid"}}}"#);
        assert!(result.is_err());
        insta::assert_snapshot!(
            "error_deeply_nested_invalid",
            result.unwrap_err().to_string()
        );

        // Array where object expected
        let result: Result<Request, _> = serde_json::from_str(r#"{"Toggle": ["workspace", "1"]}"#);
        assert!(result.is_err());
        insta::assert_snapshot!(
            "error_array_instead_of_object",
            result.unwrap_err().to_string()
        );
    }
}
