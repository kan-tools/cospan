//! End-to-end MCP stdio smoke (S5): spawn the real `cospan mcp` server and drive
//! the JSON-RPC handshake (initialize → tools/list → tools/call). Needs only the
//! built binary and a temp dir (no kan/git), so it runs in CI. Sequential
//! request→response, since rmcp services requests concurrently.

use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::process::{Child, ChildStdin, Command, Stdio};

fn send(stdin: &mut ChildStdin, req: &str) {
    writeln!(stdin, "{req}").unwrap();
    stdin.flush().unwrap();
}

/// Read lines until a JSON-RPC response with `id` arrives (skips anything else).
fn read_resp(reader: &mut impl BufRead, id: i64) -> Value {
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader.read_line(&mut line).expect("read stdout");
        assert!(n > 0, "server closed stdout before response id {id}");
        if let Ok(v) = serde_json::from_str::<Value>(line.trim()) {
            if v.get("id").and_then(|i| i.as_i64()) == Some(id) {
                return v;
            }
        }
    }
}

/// The JSON a tool returned, parsed out of the `tools/call` result's text content.
fn tool_json(resp: &Value) -> Value {
    let text = resp["result"]["content"][0]["text"]
        .as_str()
        .expect("tool result text");
    serde_json::from_str(text).expect("tool returned JSON text")
}

#[test]
fn mcp_stdio_add_and_list_round_trip() {
    let dir = std::env::temp_dir().join(format!("cospan-mcp-smoke-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("src/a.rs"), "l0\nl1\nl2\n").unwrap();

    let mut child: Child = Command::new(env!("CARGO_BIN_EXE_cospan"))
        .arg("mcp")
        .arg(&dir)
        .env("KAN_AGENT", "claude-code:tester")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn cospan mcp");
    let mut stdin = child.stdin.take().unwrap();
    let mut reader = BufReader::new(child.stdout.take().unwrap());

    // Handshake.
    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"t","version":"0"}}}"#,
    );
    let init = read_resp(&mut reader, 1);
    assert!(init["result"]["capabilities"].is_object(), "init: {init}");
    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
    );

    // tools/list advertises our five comment tools.
    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#,
    );
    let tools = read_resp(&mut reader, 2);
    let names: Vec<&str> = tools["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|t| t["name"].as_str())
        .collect();
    for want in [
        "list_comments",
        "get_thread",
        "add_comment",
        "reply",
        "resolve",
    ] {
        assert!(
            names.contains(&want),
            "tools/list missing {want}: {names:?}"
        );
    }

    // add_comment records an anchored comment at line 2 (await it before listing).
    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"add_comment","arguments":{"file":"src/a.rs","line":2,"body":"hot?"}}}"#,
    );
    let added = tool_json(&read_resp(&mut reader, 3));
    assert_eq!(added["state"], "anchored");
    assert_eq!(added["line"], 2);

    // list_comments returns it, authored by the KAN_AGENT-tagged agent.
    send(
        &mut stdin,
        r#"{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"list_comments","arguments":{"file":"src/a.rs"}}}"#,
    );
    let listed = tool_json(&read_resp(&mut reader, 4));
    let items = listed["comments"].as_array().unwrap();
    assert_eq!(items.len(), 1, "list after add: {listed}");
    assert_eq!(items[0]["body"], "hot?");
    assert_eq!(items[0]["author"]["who"], "agent");
    assert_eq!(items[0]["author"]["id"], "claude-code:tester");

    drop(stdin); // EOF -> the server exits
    let _ = child.wait();
    std::fs::remove_dir_all(&dir).ok();
}
