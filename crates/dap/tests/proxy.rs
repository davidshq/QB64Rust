//! The proxy against a scripted backend (design D2 of `m5-dap-proxy`): the test plays both the client and a stand-in
//! for `lldb-dap`, over pipes, with the proxy's `serve` on a thread between them.

mod support;

use serde_json::json;
use support::{Session, start};

#[test]
fn a_request_and_its_response_pass_with_seqs_mapped() {
    let Session {
        mut client,
        mut backend,
        ..
    } = start();
    client.send(json!({"seq": 7, "type": "request", "command": "modules", "arguments": {"count": 3}}));
    let r = backend.recv();
    assert_eq!(r["command"], "modules");
    assert_eq!(r["arguments"], json!({"count": 3}));
    assert_eq!(r["seq"], 1, "numbered by the proxy");
    backend.send(
        json!({"seq": 90, "type": "response", "request_seq": 1, "command": "modules", "success": true,
                        "body": {"modules": []}}),
    );
    let a = client.recv();
    assert_eq!(a["request_seq"], 7);
    assert_eq!(a["seq"], 1);
    assert_eq!(a["body"], json!({"modules": []}));
}

#[test]
fn an_event_passes() {
    let Session {
        mut client,
        mut backend,
        ..
    } = start();
    backend.send(json!({"seq": 33, "type": "event", "event": "output", "body": {"output": "hi\n"}}));
    let e = client.recv();
    assert_eq!(e["event"], "output");
    assert_eq!(e["body"]["output"], "hi\n");
    assert_eq!(e["seq"], 1);
    let _ = &mut client;
}

#[test]
fn a_reverse_request_and_its_answer_map_back() {
    let Session {
        mut client,
        mut backend,
        ..
    } = start();
    backend.send(json!({"seq": 50, "type": "request", "command": "runInTerminal",
                        "arguments": {"kind": "integrated", "args": ["x.exe"]}}));
    let r = client.recv();
    assert_eq!(r["command"], "runInTerminal");
    let seq = r["seq"].as_i64().unwrap();
    client.send(
        json!({"seq": 4, "type": "response", "request_seq": seq, "command": "runInTerminal",
                       "success": true, "body": {"processId": 12}}),
    );
    let a = backend.recv();
    assert_eq!(a["request_seq"], 50);
    assert_eq!(a["body"]["processId"], 12);
}

#[test]
fn initialize_adds_the_filter_and_set_next_line() {
    let Session {
        mut client,
        mut backend,
        ..
    } = start();
    client.send(json!({"seq": 1, "type": "request", "command": "initialize", "arguments": {"adapterID": "qb64"}}));
    let r = backend.recv();
    backend.send(
        json!({"seq": 1, "type": "response", "request_seq": r["seq"], "command": "initialize",
                        "success": true, "body": {
                            "supportsDataBreakpoints": true,
                            "exceptionBreakpointFilters": [{"filter": "cpp_throw", "label": "C++ Throw"}]}}),
    );
    let body = client.recv()["body"].clone();
    assert_eq!(body["supportsDataBreakpoints"], true, "lldb-dap's own kept");
    assert_eq!(body["supportsGotoTargetsRequest"], true);
    assert_eq!(body["supportsExceptionInfoRequest"], true);
    assert_eq!(
        body["exceptionBreakpointFilters"],
        json!([{"filter": "qbError", "label": "BASIC runtime errors", "default": false}])
    );
}

#[test]
fn the_backends_end_sends_terminated() {
    let Session {
        mut client,
        backend,
        done,
    } = start();
    drop(backend);
    assert_eq!(client.recv()["event"], "terminated");
    assert_eq!(done.join().unwrap(), Ok(()));
    let _ = &mut client;
}

#[test]
fn terminated_is_sent_once() {
    let Session {
        client,
        mut backend,
        done,
    } = start();
    backend.send(json!({"seq": 1, "type": "event", "event": "terminated", "body": {}}));
    assert_eq!(client.recv()["event"], "terminated");
    drop(backend);
    assert_eq!(done.join().unwrap(), Ok(()));
    assert!(client.try_recv().is_none(), "no second terminated");
}

#[test]
fn a_missing_lldb_dap_gives_the_error_naming_its_path() {
    let path = std::env::temp_dir().join("no-such-clone").join("lldb-dap.exe");
    let (mut client, done) = support::start_run(Ok(path.clone()));
    client.send(json!({"seq": 1, "type": "request", "command": "initialize", "arguments": {}}));
    let a = client.recv();
    assert_eq!(a["success"], false);
    let message = a["message"].as_str().unwrap();
    assert!(message.contains(&path.display().to_string()), "{message}");
    assert_eq!(a["body"]["error"]["showUser"], true);
    client.send(json!({"seq": 2, "type": "request", "command": "disconnect", "arguments": {}}));
    assert_eq!(client.recv()["success"], true);
    assert_eq!(done.join().unwrap(), Ok(()));
}

#[test]
fn no_clone_gives_its_error() {
    let (mut client, _done) = support::start_run(Err("QB64pe reference clone not found".into()));
    client.send(json!({"seq": 1, "type": "request", "command": "initialize", "arguments": {}}));
    assert_eq!(client.recv()["message"], "QB64pe reference clone not found");
}
