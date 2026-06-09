use rs_mcp::{handle_request, PUBLIC_IP_TOOL, WEATHER_NOW_TOOL};
use serde_json::json;

#[test]
fn tools_list_exposes_online_tools() {
    let response = handle_request(&json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/list"
    }));

    let tools = response["result"]["tools"].as_array().unwrap();
    let names = tools
        .iter()
        .map(|tool| tool["name"].as_str().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(names, vec![WEATHER_NOW_TOOL, PUBLIC_IP_TOOL]);
}

#[test]
fn weather_now_requires_city() {
    let response = handle_request(&json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/call",
        "params": {
            "name": WEATHER_NOW_TOOL,
            "arguments": {}
        }
    }));

    assert_eq!(
        response["error"]["message"],
        "Missing string argument: city"
    );
}

#[test]
fn removed_github_tool_is_unknown() {
    let response = handle_request(&json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "github_repo_summary",
            "arguments": {}
        }
    }));

    assert_eq!(response["error"]["message"], "Unknown tool");
}

#[test]
fn removed_note_tools_are_unknown() {
    let response = handle_request(&json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "create_note",
            "arguments": {}
        }
    }));

    assert_eq!(response["error"]["message"], "Unknown tool");
}
