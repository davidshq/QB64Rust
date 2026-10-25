//! Variables, evaluate, set value, data breakpoints and set next line against a scripted backend (tasks 4.5 and 4.6
//! of `m5-dap-proxy`, designs D7 and D8): the fixture `kinds` stopped in `SUB show` at line 24, called from `SUB
//! outer` at line 17, called from the main module at line 11, with each variable's memory in the [`Fake`]'s model.

mod support;

use serde_json::{Value, json};
use support::{Fake, Launched, THREAD, bas, frame, launched};

/// The model's memory: each C name's pointer and the bytes behind it.
fn memory(fake: &mut Fake) {
    let p = &mut fake.pointers;
    for (name, at) in [
        ("__LONG_TOTAL", 0x1000),
        ("__STRING_S", 0x2000),
        ("__ARRAY_SINGLE_A", 0x3000),
        ("__UDT_P", 0x4000),
        ("_SUB_SHOW_LONG_N", 0x5000),
        ("_SUB_SHOW_INTEGER_CALLS", 0x5100),
        ("_SUB_SHOW_SINGLE_X", 0x5200),
        ("_SUB_OUTER_LONG_K", 0x5300),
        ("_SUB_OUTER_LONG_Y", 0x5400),
    ] {
        p.insert(name.to_string(), at);
    }
    fake.put(0x1000, &7i32.to_le_bytes());
    // A `qbs`: `chr` at 0, `len` at 8.
    let mut qbs = 0x2100u64.to_le_bytes().to_vec();
    qbs.extend(2u32.to_le_bytes());
    qbs.extend([0; 4]);
    fake.put(0x2000, &qbs);
    fake.put(0x2100, b"hi");
    // The descriptor of `a(1 TO 3)`: the data pointer at slot 0, the one dimension from slot 4 (lower bound 1,
    // count 3, multiplier 1).
    let slots: [u64; 8] = [0x3100, 0, 2, 0, 1, 3, 1, 0];
    fake.put(0x3000, &slots.iter().flat_map(|s| s.to_le_bytes()).collect::<Vec<u8>>());
    fake.put(
        0x3100,
        &[0f32, 1.5, 0.0]
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect::<Vec<u8>>(),
    );
    let mut pt = 3i16.to_le_bytes().to_vec();
    pt.extend(b"abcd");
    fake.put(0x4000, &pt);
    fake.put(0x5000, &10i32.to_le_bytes());
    fake.put(0x5100, &1i16.to_le_bytes());
    fake.put(0x5200, &10.5f32.to_le_bytes());
    fake.put(0x5300, &5i32.to_le_bytes());
    fake.put(0x5400, &10i32.to_le_bytes());
}

/// The session stopped at the breakpoint in `SUB show`, its stack listed once (so that the frames are known).
fn stopped_in_show() -> Launched {
    let mut l = launched(false);
    memory(&mut l.fake);
    let b = bas(&l);
    l.fake.frames = vec![
        frame(1, "SUB_SHOW(int*)", &b, 24, 1),
        frame(2, "SUB_OUTER(int*)", &b, 17, 5),
        frame(3, "QBMAIN(void*)", &b, 11, 5),
    ];
    l.backend.send(json!({"seq": 50, "type": "event", "event": "stopped",
                          "body": {"reason": "breakpoint", "threadId": THREAD, "hitBreakpointIds": [99]}}));
    // As a client does, the stack is asked for after the stop is seen.
    l.stop();
    request(&mut l, "stackTrace", json!({"threadId": THREAD}));
    l
}

/// Sends a request and gives its response, answering the backend meanwhile.
fn request(l: &mut Launched, command: &str, arguments: Value) -> Value {
    l.request(command, arguments)
}

/// The variables of a reference: (name, value, type, reference).
fn variables(l: &mut Launched, reference: &Value) -> Vec<(String, String, String, i64)> {
    let r = request(l, "variables", json!({"variablesReference": reference}));
    r["body"]["variables"]
        .as_array()
        .unwrap_or_else(|| panic!("{r}"))
        .iter()
        .map(|v| {
            (
                v["name"].as_str().unwrap().to_string(),
                v["value"].as_str().unwrap().to_string(),
                v["type"].as_str().unwrap_or("").to_string(),
                v["variablesReference"].as_i64().unwrap(),
            )
        })
        .collect()
}

fn pairs(v: &[(String, String, String, i64)]) -> Vec<(&str, &str)> {
    v.iter().map(|x| (x.0.as_str(), x.1.as_str())).collect()
}

#[test]
fn scopes_and_each_kind_of_variable() {
    let mut l = stopped_in_show();
    let scopes = request(&mut l, "scopes", json!({"frameId": 1}));
    let s = &scopes["body"]["scopes"];
    assert_eq!(
        (s[0]["name"].as_str(), s[1]["name"].as_str()),
        (Some("Locals"), Some("Main module"))
    );
    let locals = variables(&mut l, &s[0]["variablesReference"]);
    let typed: Vec<(&str, &str, &str)> = locals
        .iter()
        .map(|v| (v.0.as_str(), v.1.as_str(), v.2.as_str()))
        .collect();
    assert_eq!(
        typed,
        [("N", "10", "LONG"), ("CALLS", "1", "INTEGER"), ("X", "10.5", "SINGLE")]
    );
    let main = variables(&mut l, &s[1]["variablesReference"]);
    assert_eq!(
        pairs(&main),
        [
            ("TOTAL", "7"),
            ("S", "\"hi\""),
            ("A", "(1 TO 3) AS SINGLE"),
            ("P", "PT")
        ]
    );
    let a = variables(&mut l, &json!(main[2].3));
    assert_eq!(pairs(&a), [("A(1)", "0"), ("A(2)", "1.5"), ("A(3)", "0")]);
    let p = variables(&mut l, &json!(main[3].3));
    assert_eq!(pairs(&p), [("X", "3"), ("NM", "\"abcd\"")]);
}

#[test]
fn a_frame_further_down() {
    let mut l = stopped_in_show();
    let scopes = request(&mut l, "scopes", json!({"frameId": 2}));
    let locals = variables(&mut l, &scopes["body"]["scopes"][0]["variablesReference"]);
    assert_eq!(pairs(&locals), [("K", "5"), ("Y", "10")]);
    // The main module's frame has no locals.
    let scopes = request(&mut l, "scopes", json!({"frameId": 3}));
    assert_eq!(scopes["body"]["scopes"][0]["variablesReference"], 0);
}

#[test]
fn not_allocated_yet() {
    let mut l = stopped_in_show();
    l.fake.pointers.insert("_SUB_SHOW_SINGLE_X".into(), 0);
    l.fake.pointers.remove("_SUB_SHOW_INTEGER_CALLS");
    let scopes = request(&mut l, "scopes", json!({"frameId": 1}));
    let locals = variables(&mut l, &scopes["body"]["scopes"][0]["variablesReference"]);
    assert_eq!(locals[1].1, "not allocated yet", "an unknown name");
    assert_eq!(locals[2].1, "not allocated yet", "a null pointer");
    // A descriptor with no data.
    l.fake.put(0x3000, &[0u8; 64]);
    let r = request(
        &mut l,
        "evaluate",
        json!({"expression": "a", "frameId": 1, "context": "watch"}),
    );
    assert_eq!(r["body"]["result"], "not allocated yet");
}

#[test]
fn the_references_go_when_the_program_runs() {
    let mut l = stopped_in_show();
    let scopes = request(&mut l, "scopes", json!({"frameId": 1}));
    let reference = scopes["body"]["scopes"][1]["variablesReference"].clone();
    request(&mut l, "continue", json!({"threadId": THREAD}));
    let r = request(&mut l, "variables", json!({"variablesReference": reference}));
    assert_eq!(r["success"], false);
}

fn evaluate(l: &mut Launched, expression: &str, context: &str) -> Value {
    request(
        l,
        "evaluate",
        json!({"expression": expression, "frameId": 1, "context": context}),
    )
}

#[test]
fn evaluate_names_elements_members_and_display_suffixes() {
    let mut l = stopped_in_show();
    assert_eq!(
        evaluate(&mut l, "total", "hover")["body"]["result"],
        "7",
        "the main module's, seen from the SUB"
    );
    assert_eq!(
        evaluate(&mut l, "TOTAL&", "watch")["body"]["result"],
        "7",
        "with its suffix"
    );
    assert_eq!(evaluate(&mut l, "a(2)", "watch")["body"]["result"], "1.5");
    assert_eq!(evaluate(&mut l, "p.nm", "hover")["body"]["result"], "\"abcd\"");
    assert_eq!(evaluate(&mut l, "n,h", "watch")["body"]["result"], "&HA");
    assert_eq!(evaluate(&mut l, "n,b", "watch")["body"]["result"], "&B1010");
    let hex = request(
        &mut l,
        "evaluate",
        json!({"expression": "n", "frameId": 1, "context": "watch", "format": {"hex": true}}),
    );
    assert_eq!(hex["body"]["result"], "&HA");
    let p = evaluate(&mut l, "p", "watch");
    assert!(
        p["body"]["variablesReference"].as_i64().unwrap() > 0,
        "a user type expands: {p}"
    );
    let none = evaluate(&mut l, "nosuch", "hover");
    assert_eq!(
        (none["success"].clone(), none["message"].clone()),
        (json!(false), json!("not a variable here"))
    );
    let range = evaluate(&mut l, "a(4)", "watch");
    assert!(range["message"].as_str().unwrap().contains("out of range"), "{range}");
    // A suffix of another type names nothing.
    assert_eq!(evaluate(&mut l, "total%", "watch")["success"], false);
}

#[test]
fn a_procedures_own_variable_first() {
    let mut l = stopped_in_show();
    // `x` is SUB show's own SINGLE.
    assert_eq!(evaluate(&mut l, "x", "hover")["body"]["result"], "10.5");
    // From SUB outer, `x` is no variable.
    let r = request(
        &mut l,
        "evaluate",
        json!({"expression": "x", "frameId": 2, "context": "hover"}),
    );
    assert_eq!(r["success"], false);
}

#[test]
fn the_debug_console_passes_what_is_no_variable_to_lldb() {
    let mut l = stopped_in_show();
    l.client
        .send(json!({"seq": 31, "type": "request", "command": "evaluate",
                         "arguments": {"expression": "`bt", "frameId": 1, "context": "repl"}}));
    let m = l.fake.until(&mut l.backend, "evaluate");
    assert_eq!(m["arguments"]["expression"], "`bt");
    l.fake.answer(&mut l.backend, &m);
    assert_eq!(l.client.recv()["request_seq"], 31);
}

#[test]
fn the_debug_console_may_write_memory_so_it_is_read_again() {
    let mut l = stopped_in_show();
    assert_eq!(evaluate(&mut l, "total", "watch")["body"]["result"], "7");
    // An lldb command writes TOTAL behind the proxy's back.
    l.fake.put(0x1000, &9i32.to_le_bytes());
    evaluate(&mut l, "`expr *__LONG_TOTAL = 9", "repl");
    assert_eq!(evaluate(&mut l, "total", "watch")["body"]["result"], "9");
}

#[test]
fn lldb_dap_references_from_2_to_the_32_pass_through() {
    let mut l = stopped_in_show();
    let lasting = (1i64 << 32) + 1;
    l.client
        .send(json!({"seq": 31, "type": "request", "command": "variables",
                         "arguments": {"variablesReference": lasting}}));
    let m = l.fake.until(&mut l.backend, "variables");
    assert_eq!(m["arguments"]["variablesReference"], lasting);
    l.fake.answer(&mut l.backend, &m);
    assert_eq!(l.client.recv()["request_seq"], 31);
}

#[test]
fn bounds_of_garbage_fail_to_read() {
    let mut l = stopped_in_show();
    // A descriptor not set up yet: a count near the largest integer.
    let slots: [u64; 8] = [0x3100, 0, 2, 0, 0, i64::MAX.cast_unsigned(), 1, 0];
    l.fake
        .put(0x3000, &slots.iter().flat_map(|s| s.to_le_bytes()).collect::<Vec<u8>>());
    let r = evaluate(&mut l, "a(9223372036854775806)", "watch");
    assert_eq!(r["success"], false);
    assert!(r["message"].as_str().unwrap().contains("not valid"), "{r}");
    // A `qbs` whose text pointer is garbage, once the page before it was read.
    let mut qbs = u64::MAX.to_le_bytes().to_vec();
    qbs.extend(2u32.to_le_bytes());
    qbs.extend([0; 4]);
    l.fake.put(0x2000, &qbs);
    assert_eq!(evaluate(&mut l, "total", "watch")["body"]["result"], "7");
    assert_eq!(evaluate(&mut l, "s", "watch")["body"]["result"], "not allocated yet");
}

/// The fixture `twins`: `DIM a(1 TO 3) AS SINGLE` and a scalar `a` of the same name and type.
#[test]
fn a_scalar_and_an_array_of_one_name() {
    let mut l = support::launched_on("twins", false);
    let path = l.dir.path().join("twins.bas").to_string_lossy().to_string();
    l.fake.pointers.insert("__ARRAY_SINGLE_A".into(), 0x3000);
    l.fake.pointers.insert("__SINGLE_A".into(), 0x1000);
    l.fake.put(0x1000, &5f32.to_le_bytes());
    let slots: [u64; 8] = [0x3100, 0, 2, 0, 1, 3, 1, 0];
    l.fake
        .put(0x3000, &slots.iter().flat_map(|s| s.to_le_bytes()).collect::<Vec<u8>>());
    l.fake.put(
        0x3100,
        &[0f32, 1.5, 0.0]
            .iter()
            .flat_map(|f| f.to_le_bytes())
            .collect::<Vec<u8>>(),
    );
    l.fake.frames = vec![frame(1, "QBMAIN(void*)", &path, 5, 1)];
    l.backend.send(json!({"seq": 50, "type": "event", "event": "stopped",
                          "body": {"reason": "breakpoint", "threadId": THREAD, "hitBreakpointIds": [99]}}));
    l.stop();
    request(&mut l, "stackTrace", json!({"threadId": THREAD}));
    assert_eq!(evaluate(&mut l, "a", "hover")["body"]["result"], "5");
    assert_eq!(evaluate(&mut l, "a(2)", "hover")["body"]["result"], "1.5");
}

#[test]
fn set_value() {
    let mut l = stopped_in_show();
    let r = request(
        &mut l,
        "setExpression",
        json!({"expression": "total", "value": "42", "frameId": 1}),
    );
    assert_eq!(r["body"]["value"], "42");
    assert_eq!(
        l.fake.memory.iter().rev().find(|(a, _)| *a == 0x1000).unwrap().1,
        42i32.to_le_bytes()
    );
    let bad = request(
        &mut l,
        "setExpression",
        json!({"expression": "calls", "value": "40000", "frameId": 1}),
    );
    assert_eq!(bad["success"], false);
    assert!(bad["message"].as_str().unwrap().contains("out of the range"), "{bad}");
    let nan = request(
        &mut l,
        "setExpression",
        json!({"expression": "calls", "value": "abc", "frameId": 1}),
    );
    assert_eq!(nan["success"], false);
    // A STRING * 4 in place, padded with spaces.
    let r = request(
        &mut l,
        "setExpression",
        json!({"expression": "p.nm", "value": "\"xy\"", "frameId": 1}),
    );
    assert_eq!(r["body"]["value"], "\"xy  \"");
    // A STRING through libqb's own assignment.
    request(
        &mut l,
        "setExpression",
        json!({"expression": "s", "value": "\"ab\"", "frameId": 1}),
    );
    let call = l
        .fake
        .seen
        .iter()
        .rev()
        .find(|m| {
            m["arguments"]["expression"]
                .as_str()
                .is_some_and(|e| e.contains("qbs_set"))
        })
        .unwrap();
    let e = call["arguments"]["expression"].as_str().unwrap();
    assert!(
        e.contains("(void*)8192") && e.contains("\\141\\142") && e.contains(", 2))"),
        "{e}"
    );
    // setVariable through a scope's reference.
    let scopes = request(&mut l, "scopes", json!({"frameId": 1}));
    let r = request(
        &mut l,
        "setVariable",
        json!({"variablesReference": scopes["body"]["scopes"][0]["variablesReference"],
                                                   "name": "X", "value": "2.25"}),
    );
    assert_eq!(r["body"]["value"], "2.25");
}

#[test]
fn data_breakpoints_up_to_8_bytes() {
    let mut l = stopped_in_show();
    let r = request(&mut l, "dataBreakpointInfo", json!({"name": "total", "frameId": 1}));
    assert_eq!(r["body"]["dataId"], "1000/4");
    let r = request(&mut l, "dataBreakpointInfo", json!({"name": "p.nm", "frameId": 1}));
    assert_eq!(r["body"]["dataId"], "4002/4");
    let r = request(&mut l, "dataBreakpointInfo", json!({"name": "s", "frameId": 1}));
    assert!(r["body"]["dataId"].is_null());
    assert!(r["body"]["description"].as_str().unwrap().contains("STRING"), "{r}");
    let scopes = request(&mut l, "scopes", json!({"frameId": 1}));
    let r = request(
        &mut l,
        "dataBreakpointInfo",
        json!({"variablesReference": scopes["body"]["scopes"][0]["variablesReference"],
                                                         "name": "CALLS"}),
    );
    assert_eq!(r["body"]["dataId"], "5100/2");
}

#[test]
fn set_next_line_within_the_procedure() {
    let mut l = stopped_in_show();
    let source = json!({"path": bas(&l)});
    let t = request(&mut l, "gotoTargets", json!({"source": source, "line": 22}));
    assert_eq!(t["body"]["targets"][0]["line"], 22);
    for outside in [12, 17, 19, 30] {
        let t = request(&mut l, "gotoTargets", json!({"source": source, "line": outside}));
        assert_eq!(t["body"]["targets"], json!([]), "line {outside}");
    }
    request(&mut l, "gotoTargets", json!({"source": source, "line": 22}));
    l.client.send(
        json!({"seq": 40, "type": "request", "command": "goto", "arguments": {"threadId": THREAD, "targetId": 22}}),
    );
    let m = l.fake.until(&mut l.backend, "evaluate");
    assert_eq!(
        m["arguments"]["expression"],
        "`thread jump --file \"kinds.bas\" --line 22"
    );
    assert_eq!(m["arguments"]["context"], "repl");
    l.fake.answer(&mut l.backend, &m);
    assert_eq!(l.client.recv()["success"], true);
    let s = l.client.recv();
    assert_eq!(
        (s["event"].clone(), s["body"]["reason"].clone()),
        (json!("stopped"), json!("goto"))
    );
}

/// The fixture `bits`: a `_BIT * 3`, a `_MEM` and `big(1 TO 1500) AS SINGLE`, stopped in the main module.
#[test]
fn bits_mems_and_an_array_past_1000_elements() {
    let mut l = support::launched_on("bits", false);
    let path = l.dir.path().join("bits.bas").to_string_lossy().to_string();
    l.fake.pointers.insert("__BIT3_B".into(), 0x1000);
    l.fake.pointers.insert("__UDT_M".into(), 0x2000);
    l.fake.pointers.insert("__ARRAY_SINGLE_BIG".into(), 0x3000);
    l.fake.put(0x1000, &[1, 0, 0, 0]);
    l.fake.put(0x2000, &[0; 64]);
    let slots: [u64; 8] = [0x4000, 0, 2, 0, 1, 1500, 1, 0];
    l.fake
        .put(0x3000, &slots.iter().flat_map(|s| s.to_le_bytes()).collect::<Vec<u8>>());
    l.fake.put(0x4000, &vec![0u8; 1500 * 4]);
    l.fake.frames = vec![frame(1, "QBMAIN(void*)", &path, 5, 1)];
    l.backend.send(json!({"seq": 50, "type": "event", "event": "stopped",
                          "body": {"reason": "breakpoint", "threadId": THREAD, "hitBreakpointIds": [99]}}));
    l.stop();
    request(&mut l, "stackTrace", json!({"threadId": THREAD}));
    let scopes = request(&mut l, "scopes", json!({"frameId": 1}));
    let main = variables(&mut l, &scopes["body"]["scopes"][1]["variablesReference"]);
    assert_eq!(main[0].1, "not shown yet (_BIT)");
    assert_eq!(main[1].1, "not shown yet (_MEM)");
    assert_eq!(main[2].1, "(1 TO 1500) AS SINGLE");
    let elements = variables(&mut l, &json!(main[2].3));
    assert_eq!(elements.len(), 1001, "1000 elements, then a note");
    assert_eq!(elements[999].0, "BIG(1000)");
    assert!(elements[1000].0.contains("500 more"), "{:?}", elements[1000]);
}

#[test]
fn set_next_line_in_the_main_module() {
    let mut l = launched(false);
    let b = bas(&l);
    l.fake.frames = vec![frame(3, "QBMAIN(void*)", &b, 11, 1)];
    l.backend.send(json!({"seq": 50, "type": "event", "event": "stopped",
                          "body": {"reason": "breakpoint", "threadId": THREAD, "hitBreakpointIds": [99]}}));
    l.stop();
    let source = json!({"path": b});
    let t = request(&mut l, "gotoTargets", json!({"source": source, "line": 10}));
    assert_eq!(t["body"]["targets"][0]["line"], 10);
    let t = request(&mut l, "gotoTargets", json!({"source": source, "line": 16}));
    assert_eq!(t["body"]["targets"], json!([]), "a line of SUB outer");
}
