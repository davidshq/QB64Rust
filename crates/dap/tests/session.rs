//! The proxy's rewrites of a session against a scripted backend (tasks 4.2–4.4 and 4.7 of `m5-dap-proxy`): launch
//! and its own breakpoints (D4, D9), the stack and the one thread (D5, D11), stepping (D6), the error stop (D10). The
//! program is the fixture `tests\fixtures\kinds.bas` with the symbol file the compiler writes for it.

mod support;

use serde_json::{Value, json};
use support::{Launched, Session, THREAD, bas, frame, launched, start};

/// The proxy's bound on the steps of one client step (`step.rs`).
const MAX_STEPS: u32 = 200;

fn stopped(reason: &str, hits: &[i64]) -> Value {
    json!({"seq": 50, "type": "event", "event": "stopped",
           "body": {"reason": reason, "threadId": THREAD, "hitBreakpointIds": hits}})
}

/// The stop the client is sent next, skipping other events.
fn client_stop(l: &mut Launched) -> Value {
    l.stop()
}

#[test]
fn launch_without_a_symbol_file_is_refused_and_starts_nothing() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("plain.exe"), b"").unwrap();
    let Session {
        mut client, backend, ..
    } = start();
    client.send(json!({"seq": 1, "type": "request", "command": "launch",
                       "arguments": {"program": dir.path().join("plain.exe")}}));
    let r = client.recv();
    assert_eq!(r["success"], false);
    let message = r["message"].as_str().unwrap();
    assert!(
        message.contains("plain.exe.debug.json") && message.contains("--debug"),
        "{message}"
    );
    assert!(backend.try_recv().is_none(), "no launch reached lldb-dap");
}

#[test]
fn launch_of_another_version_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("p.exe"), b"").unwrap();
    std::fs::write(dir.path().join("p.exe.debug.json"), br#"{"version": 2}"#).unwrap();
    let Session {
        mut client,
        backend: _backend,
        ..
    } = start();
    client.send(
        json!({"seq": 1, "type": "request", "command": "launch", "arguments": {"program": dir.path().join("p.exe")}}),
    );
    let message = client.recv()["message"].as_str().unwrap().to_string();
    assert!(
        message.contains("version 2") && message.contains("--debug"),
        "{message}"
    );
}

/// An executable newer than its symbol file was built again by another build (QB64pe's): refused, as one without.
#[test]
fn launch_of_an_executable_newer_than_its_symbol_file_is_refused() {
    let dir = tempfile::tempdir().unwrap();
    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    std::fs::copy(
        fixtures.join("kinds.exe.debug.json"),
        dir.path().join("kinds.exe.debug.json"),
    )
    .unwrap();
    let exe = std::fs::File::create(dir.path().join("kinds.exe")).unwrap();
    exe.set_modified(std::time::SystemTime::now() + std::time::Duration::from_secs(60))
        .unwrap();
    drop(exe);
    let Session {
        mut client, backend, ..
    } = start();
    client.send(json!({"seq": 1, "type": "request", "command": "launch",
                       "arguments": {"program": dir.path().join("kinds.exe")}}));
    let r = client.recv();
    assert_eq!(r["success"], false);
    let message = r["message"].as_str().unwrap();
    assert!(
        message.contains("newer than its debug symbol file") && message.contains("--debug"),
        "{message}"
    );
    assert!(backend.try_recv().is_none(), "no launch reached lldb-dap");
}

#[test]
fn the_backends_launch() {
    let dir = tempfile::tempdir().unwrap();
    let fixtures = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");
    // The executable first: its symbol file must not be older (`launch`), and a copy keeps the fixture's time.
    std::fs::write(dir.path().join("kinds.exe"), b"").unwrap();
    std::fs::write(
        dir.path().join("kinds.exe.debug.json"),
        std::fs::read(fixtures.join("kinds.exe.debug.json")).unwrap(),
    )
    .unwrap();
    let Session {
        mut client, backend, ..
    } = start();
    client.send(json!({"seq": 1, "type": "request", "command": "launch", "arguments": {
        "program": dir.path().join("kinds.exe"), "args": ["a b"], "noPrompt": true, "stopOnEntry": true,
        "console": "integratedTerminal", "env": {"X": "1"}}}));
    let l = backend.recv();
    let a = &l["arguments"];
    assert_eq!(a["args"], json!(["a b"]));
    assert_eq!(a["env"], json!({"X": "1", "QB64PE_NOPROMPT": "y"}));
    assert_eq!(a["stopOnEntry"], false, "the proxy's own stop on entry");
    assert_eq!(a["cwd"], json!(dir.path()), "the executable's folder by default");
    assert_eq!(a["runInTerminal"], true);
    let _ = &mut client;
}

#[test]
fn function_breakpoints_are_the_union_and_the_client_sees_its_own() {
    let mut l = launched(false);
    l.fake.through(&mut l.backend, "continue");
    l.client
        .send(json!({"seq": 4, "type": "request", "command": "setFunctionBreakpoints",
                         "arguments": {"breakpoints": [{"name": "SUB_SHOW"}]}}));
    let set = l.fake.through(&mut l.backend, "setFunctionBreakpoints");
    assert_eq!(
        set["arguments"]["breakpoints"],
        json!([{"name": "SUB_SHOW"}]),
        "QBMAIN's is gone after its hit"
    );
    let r = l.client.recv();
    assert_eq!(r["body"]["breakpoints"].as_array().unwrap().len(), 1);
    // The filter adds the proxy's breakpoint on `error`; the client's answer lists only its own.
    l.client.send(
        json!({"seq": 5, "type": "request", "command": "setExceptionBreakpoints",
                         "arguments": {"filters": ["qbError"]}}),
    );
    let set = l.fake.through(&mut l.backend, "setFunctionBreakpoints");
    assert_eq!(
        set["arguments"]["breakpoints"],
        json!([{"name": "SUB_SHOW"}, {"name": "error", "condition": "(int)$ecx != 0"}])
    );
    assert_eq!(l.client.recv()["success"], true);
    let own = l.fake.next_id;
    // An event about the proxy's own breakpoint is not passed; one about the client's is.
    l.backend.send(json!({"seq": 60, "type": "event", "event": "breakpoint",
                          "body": {"reason": "changed", "breakpoint": {"id": own}}}));
    l.backend.send(json!({"seq": 61, "type": "event", "event": "breakpoint",
                          "body": {"reason": "changed", "breakpoint": {"id": own - 1}}}));
    assert_eq!(l.client.recv()["body"]["breakpoint"]["id"], own - 1);
}

#[test]
fn stop_on_entry_steps_to_the_first_statement() {
    let mut l = launched(true);
    l.fake.frames = vec![frame(1, "QBMAIN(void*)", "build\\qbx.cpp", 1563, 5)];
    l.fake.through(&mut l.backend, "next");
    l.backend.send(stopped("step", &[]));
    l.fake.frames = vec![frame(1, "QBMAIN(void*)", &bas(&l), 1, 1)];
    l.fake.through(&mut l.backend, "next");
    l.backend.send(stopped("step", &[]));
    l.fake.frames = vec![frame(1, "QBMAIN(void*)", &bas(&l), 10, 1)];
    let s = client_stop(&mut l);
    assert_eq!(s["body"]["reason"], "entry");
    assert_eq!(s["body"]["threadId"], THREAD);
}

#[test]
fn the_stack_of_a_sub_under_a_sub_under_the_main_module() {
    let mut l = launched(false);
    let b = bas(&l);
    l.fake.frames = vec![
        frame(1, "SUB_SHOW(int*)", &b, 24, 1),
        frame(2, "SUB_OUTER(int*)", &b, 17, 5),
        frame(3, "QBMAIN(void*)", &b, 11, 5),
        frame(
            4,
            "stdcall_thread_wrapper(void*)",
            "build\\kinds.exe`stdcall_thread_wrapper(void*)",
            6,
            1,
        ),
    ];
    l.backend.send(stopped("breakpoint", &[99]));
    assert_eq!(client_stop(&mut l)["body"]["threadId"], THREAD);
    l.client
        .send(json!({"seq": 10, "type": "request", "command": "stackTrace", "arguments": {"threadId": THREAD}}));
    l.fake.settle(&mut l.backend);
    let st = l.client.recv();
    let names: Vec<(&str, u64)> = st["body"]["stackFrames"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["name"].as_str().unwrap(), f["line"].as_u64().unwrap()))
        .collect();
    assert_eq!(names, [("SUB show", 24), ("SUB outer", 17), ("Main module", 11)]);
    assert_eq!(st["body"]["totalFrames"], 3);
    // Paged by the client.
    l.client
        .send(json!({"seq": 11, "type": "request", "command": "stackTrace",
                         "arguments": {"threadId": THREAD, "startFrame": 1, "levels": 1}}));
    l.fake.settle(&mut l.backend);
    let st = l.client.recv();
    assert_eq!(st["body"]["stackFrames"][0]["name"], "SUB outer");
    assert_eq!(st["body"]["stackFrames"].as_array().unwrap().len(), 1);
}

#[test]
fn a_handler_above_evnt_above_the_raising_line() {
    let mut l = launched(false);
    let b = bas(&l);
    l.fake.frames = vec![
        frame(1, "QBMAIN(void*)", &b, 8, 1),
        frame(2, "evnt(int, int, int)", "build\\qbx.cpp", 1200, 5),
        frame(3, "QBMAIN(void*)", &b, 4, 19),
    ];
    l.backend.send(stopped("breakpoint", &[99]));
    client_stop(&mut l);
    l.client
        .send(json!({"seq": 10, "type": "request", "command": "stackTrace", "arguments": {"threadId": THREAD}}));
    l.fake.settle(&mut l.backend);
    let st = l.client.recv();
    let frames: Vec<(&str, u64)> = st["body"]["stackFrames"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| (f["name"].as_str().unwrap(), f["line"].as_u64().unwrap()))
        .collect();
    assert_eq!(frames, [("Main module", 8), ("Main module", 4)]);
}

#[test]
fn the_start_after_the_main_modules_breakpoint_is_not_shown() {
    let mut l = launched(false);
    l.fake.through(&mut l.backend, "continue");
    l.backend.send(json!({"seq": 51, "type": "event", "event": "continued",
                          "body": {"threadId": THREAD, "allThreadsContinued": true}}));
    l.backend
        .send(json!({"seq": 52, "type": "event", "event": "output", "body": {"output": "start\n"}}));
    assert_eq!(l.client.recv()["event"], "output", "no continued before it");
    // The client's own continue is shown.
    l.client
        .send(json!({"seq": 10, "type": "request", "command": "continue", "arguments": {"threadId": THREAD}}));
    l.fake.through(&mut l.backend, "continue");
    assert_eq!(l.client.recv()["command"], "continue");
    l.backend.send(json!({"seq": 53, "type": "event", "event": "continued",
                          "body": {"threadId": THREAD, "allThreadsContinued": true}}));
    assert_eq!(l.client.recv()["event"], "continued");
}

#[test]
fn a_failed_request_at_a_stop_shows_the_stop_and_why() {
    let mut l = launched(false);
    l.client
        .send(json!({"seq": 20, "type": "request", "command": "next", "arguments": {"threadId": THREAD}}));
    l.fake.frames = vec![json!({}); 3];
    l.fake.through(&mut l.backend, "next");
    l.client.recv();
    l.backend.send(stopped("step", &[]));
    let st = l.fake.until(&mut l.backend, "stackTrace");
    l.backend.send(
        json!({"seq": 0, "type": "response", "request_seq": st["seq"], "command": "stackTrace",
                          "success": false, "message": "the process has exited"}),
    );
    let out = l.client.recv();
    assert_eq!(out["event"], "output");
    assert!(
        out["body"]["output"]
            .as_str()
            .unwrap()
            .contains("the process has exited"),
        "{out}"
    );
    let s = l.client.recv();
    assert_eq!(
        (s["event"].clone(), s["body"]["reason"].clone()),
        (json!("stopped"), json!("step"))
    );
    assert_eq!(s["body"]["threadId"], THREAD);
    let info = l.request("exceptionInfo", json!({"threadId": THREAD}));
    assert_eq!(info["success"], false, "no error known at this stop: {info}");
}

#[test]
fn a_continued_after_a_stop_is_shown_when_the_start_had_none() {
    let mut l = launched(false);
    l.fake.through(&mut l.backend, "continue");
    // No `continued` for the proxy's own start; the program stops at a breakpoint.
    l.backend.send(stopped("breakpoint", &[99]));
    client_stop(&mut l);
    l.request("continue", json!({"threadId": THREAD}));
    l.backend.send(json!({"seq": 53, "type": "event", "event": "continued",
                          "body": {"threadId": THREAD, "allThreadsContinued": true}}));
    assert_eq!(l.wait(|m| m["type"] == "event")["event"], "continued");
}

#[test]
fn a_pause_that_brought_no_stop_does_not_name_a_later_exception() {
    let mut l = launched(false);
    l.request("pause", json!({"threadId": THREAD}));
    l.request("continue", json!({"threadId": THREAD}));
    l.backend.send(json!({"seq": 50, "type": "event", "event": "stopped",
                          "body": {"reason": "exception", "description": "Exception 0xc0000005", "threadId": 99}}));
    assert_eq!(client_stop(&mut l)["body"]["reason"], "exception");
}

#[test]
fn an_exception_on_another_thread_without_a_pause_is_shown_as_it_came() {
    let mut l = launched(false);
    l.backend.send(json!({"seq": 50, "type": "event", "event": "stopped",
                          "body": {"reason": "exception", "description": "Exception 0xc0000005", "threadId": 99}}));
    let s = client_stop(&mut l);
    assert_eq!(s["body"]["reason"], "exception");
    assert_eq!(s["body"]["description"], "Exception 0xc0000005");
    assert_eq!(s["body"]["threadId"], THREAD);
}

#[test]
fn a_pause_on_another_thread_is_the_programs() {
    let mut l = launched(false);
    l.request("pause", json!({"threadId": THREAD}));
    l.backend.send(json!({"seq": 50, "type": "event", "event": "stopped",
                          "body": {"reason": "exception", "description": "interrupt", "threadId": 99}}));
    let s = client_stop(&mut l);
    assert_eq!(s["body"]["reason"], "pause");
    assert_eq!(s["body"]["threadId"], THREAD);
    l.client
        .send(json!({"seq": 10, "type": "request", "command": "threads", "arguments": {}}));
    assert_eq!(
        l.client.recv()["body"]["threads"],
        json!([{"id": THREAD, "name": "Program"}])
    );
}

/// Sends the client's step, answers the backend's step, and plays `stops` one by one: each is the top frame of a
/// `step` stop, after which the proxy steps on with the command given, or (`None`) reports.
fn play(l: &mut Launched, command: &str, start_depth: usize, stops: &[(Value, usize, Option<&str>)]) -> Value {
    l.client
        .send(json!({"seq": 20, "type": "request", "command": command, "arguments": {"threadId": THREAD}}));
    l.fake.frames = vec![json!({}); start_depth];
    l.fake.through(&mut l.backend, command);
    assert_eq!(l.client.recv()["success"], true, "answered at once");
    for (top, depth, then) in stops {
        let mut frames = vec![top.clone()];
        frames.resize(*depth, json!({}));
        l.fake.frames = frames;
        l.backend.send(stopped("step", &[]));
        match then {
            Some(next) => {
                let m = l.fake.until(&mut l.backend, next);
                l.fake.answer(&mut l.backend, &m);
            }
            None => return client_stop(l),
        }
    }
    panic!("no report");
}

#[test]
fn step_into_a_sub_passes_its_header_and_libqbs_inline_function() {
    let mut l = launched(false);
    let b = bas(&l);
    let s = play(
        &mut l,
        "stepIn",
        3,
        &[
            (frame(1, "SUB_SHOW(int*)", &b, 19, 21), 4, Some("next")),
            (
                frame(1, "is_error_pending()", "build\\error_handle.h", 24, 5),
                5,
                Some("stepOut"),
            ),
            (frame(1, "SUB_SHOW(int*)", &b, 19, 5), 4, Some("next")),
            (frame(1, "SUB_SHOW(int*)", &b, 22, 1), 4, None),
        ],
    );
    assert_eq!(s["body"]["reason"], "step");
    assert_eq!(s["body"]["threadId"], THREAD);
}

#[test]
fn step_into_a_function_of_an_included_file_passes_its_data_fragment() {
    let mut l = launched(false);
    let b = bas(&l);
    let s = play(
        &mut l,
        "stepIn",
        3,
        &[
            (frame(1, "SUB_OUTER(int*)", "build\\data1.txt", 2, 1), 4, Some("next")),
            (frame(1, "SUB_OUTER(int*)", &b, 14, 5), 4, Some("next")),
            (frame(1, "SUB_OUTER(int*)", &b, 16, 1), 4, None),
        ],
    );
    assert_eq!(s["body"]["reason"], "step");
}

#[test]
fn step_out_steps_past_the_rest_of_the_call() {
    let mut l = launched(false);
    let b = bas(&l);
    play(
        &mut l,
        "stepOut",
        4,
        &[
            (frame(1, "SUB_OUTER(int*)", &b, 17, 5), 3, Some("next")),
            (frame(1, "SUB_OUTER(int*)", &b, 18, 1), 3, None),
        ],
    );
}

#[test]
fn next_past_a_files_end_and_evnt() {
    let mut l = launched(false);
    let b = bas(&l);
    play(
        &mut l,
        "next",
        4,
        &[
            (frame(1, "QBMAIN(void*)", &b, 40, 1), 4, Some("next")),
            (
                frame(1, "evnt(int, int, int)", "build\\qbx.cpp", 900, 1),
                3,
                Some("stepOut"),
            ),
            (frame(1, "QBMAIN(void*)", &b, 10, 19), 2, Some("next")),
            (frame(1, "QBMAIN(void*)", &b, 11, 1), 2, None),
        ],
    );
}

/// Measured (`dap_step_in_on_lines_without_a_call`): `stepIn` on `s$ = STR$(z)` enters libqb's inline
/// `qbs_cleanup` and its `stepOut` comes back to the same line at column 5, at the depth the step began at.
#[test]
fn step_in_goes_on_past_the_rest_of_a_line_after_libqbs_inline_function() {
    let mut l = launched(false);
    let b = bas(&l);
    let s = play(
        &mut l,
        "stepIn",
        2,
        &[
            (
                frame(1, "int qbs_cleanup<int>(unsigned int, int)", "build\\qbs.h", 99, 12),
                3,
                Some("stepOut"),
            ),
            (frame(1, "QBMAIN(void*)", &b, 11, 5), 2, Some("stepIn")),
            (frame(1, "QBMAIN(void*)", &b, 12, 1), 2, None),
        ],
    );
    assert_eq!(s["body"]["reason"], "step");
}

#[test]
fn a_breakpoint_during_the_steps_ends_them() {
    let mut l = launched(false);
    let b = bas(&l);
    l.client
        .send(json!({"seq": 20, "type": "request", "command": "stepIn", "arguments": {"threadId": THREAD}}));
    l.fake.through(&mut l.backend, "stepIn");
    l.client.recv();
    l.fake.frames = vec![frame(1, "is_error_pending()", "build\\error_handle.h", 24, 5)];
    l.backend.send(stopped("step", &[]));
    l.fake.through(&mut l.backend, "stepOut");
    l.fake.frames = vec![frame(1, "SUB_SHOW(int*)", &b, 23, 1)];
    l.backend.send(stopped("breakpoint", &[99]));
    let s = client_stop(&mut l);
    assert_eq!(s["body"]["reason"], "breakpoint");
    assert_eq!(s["body"]["hitBreakpointIds"], json!([99]));
}

#[test]
fn the_bound_of_200_steps() {
    let mut l = launched(false);
    l.client
        .send(json!({"seq": 20, "type": "request", "command": "next", "arguments": {"threadId": THREAD}}));
    l.fake.frames = vec![frame(1, "libqb_loop()", "build\\libqb.cpp", 1, 1)];
    l.fake.through(&mut l.backend, "next");
    l.client.recv();
    for _ in 0..MAX_STEPS {
        l.backend.send(stopped("step", &[]));
        l.fake.through(&mut l.backend, "stepOut");
    }
    l.backend.send(stopped("step", &[]));
    l.fake.settle(&mut l.backend);
    let out = l.client.recv();
    assert_eq!(out["event"], "output");
    assert!(out["body"]["output"].as_str().unwrap().contains("cut short"), "{out}");
    assert_eq!(client_stop(&mut l)["body"]["reason"], "step");
}

#[test]
fn the_error_stop_names_the_error_and_exception_info_gives_it() {
    let mut l = launched(false);
    l.fake.through(&mut l.backend, "continue");
    l.client.send(
        json!({"seq": 5, "type": "request", "command": "setExceptionBreakpoints",
                         "arguments": {"filters": ["qbError"]}}),
    );
    l.fake.through(&mut l.backend, "setFunctionBreakpoints");
    l.client.recv();
    let own = l.fake.next_id;
    l.fake.ecx = 5;
    l.fake.frames = vec![
        frame(1, "error", "build\\kinds.exe`error", 0, 0),
        frame(2, "QBMAIN(void*)", &bas(&l), 10, 1),
    ];
    l.backend.send(stopped("breakpoint", &[own]));
    let s = client_stop(&mut l);
    assert_eq!(s["body"]["reason"], "exception");
    assert_eq!(s["body"]["description"], "Error 5: Illegal function call");
    l.client
        .send(json!({"seq": 6, "type": "request", "command": "exceptionInfo", "arguments": {"threadId": THREAD}}));
    let info = l.client.recv();
    assert_eq!(info["body"]["description"], "Error 5: Illegal function call");
    assert_eq!(info["body"]["exceptionId"], "qbError");
    // Without the filter, the breakpoint goes.
    l.client
        .send(json!({"seq": 7, "type": "request", "command": "setExceptionBreakpoints", "arguments": {"filters": []}}));
    let set = l.fake.through(&mut l.backend, "setFunctionBreakpoints");
    assert_eq!(set["arguments"]["breakpoints"], json!([]));
}
