import * as assert from "assert";
import { JsonLines, listedTests, messageOf, parseEvent, runArgs, stateOf, TestResultEvent } from "../../src/compiler/testOutput";

// Lines as `qb64rust test --format json` writes them (crates\driver\src\unit_tests\report.rs).
const TEST = '{"event":"test","file":"C:\\\\ws\\\\lib\\\\math_test.bas","name":"TestAdd","line":5}';
const RESULT =
    '{"event":"result","file":"C:\\\\ws\\\\lib\\\\math_test.bas","name":"TestAdd","line":5,"outcome":"fail",' +
    '"message":"sum: expected 4, got 5","location":null,"output":"hi\\u000d\\u000a","ms":12}';
const SUMMARY = '{"event":"summary","passed":0,"failed":1,"errors":0,"timeouts":0}';

describe("testOutput: events", () => {
    it("a test found", () => {
        assert.deepStrictEqual(parseEvent(TEST), { event: "test", file: "C:\\ws\\lib\\math_test.bas", name: "TestAdd", line: 5 });
    });

    it("a result, with its output and no location", () => {
        assert.deepStrictEqual(parseEvent(RESULT), {
            event: "result",
            file: "C:\\ws\\lib\\math_test.bas",
            name: "TestAdd",
            line: 5,
            outcome: "fail",
            message: "sum: expected 4, got 5",
            location: null,
            output: "hi\r\n",
            ms: 12,
        });
    });

    it("a result with a location", () => {
        const e = parseEvent('{"event":"result","file":"a_test.bas","name":"TestA","line":3,"outcome":"fail","message":"math","location":{"file":"a_test.bas","line":9},"output":"","ms":1}');
        assert.deepStrictEqual(e?.event === "result" ? e.location : undefined, { file: "a_test.bas", line: 9 });
    });

    it("the summary and an error", () => {
        assert.deepStrictEqual(parseEvent(SUMMARY), { event: "summary", passed: 0, failed: 1, errors: 0, timeouts: 0 });
        assert.deepStrictEqual(parseEvent('{"event":"error","message":"no test found"}'), { event: "error", message: "no test found" });
    });

    it("anything else is no event", () => {
        for (const line of ["", "qb64rust: usage", "{not json", '{"event":"other"}']) {
            assert.strictEqual(parseEvent(line), undefined, line);
        }
    });

    it("lines split across chunks, with CRLF", () => {
        const lines = new JsonLines();
        const all = `${TEST}\r\n${RESULT}\r\n${SUMMARY}`;
        const events = [...lines.push(all.slice(0, 30)), ...lines.push(all.slice(30, 200)), ...lines.push(all.slice(200)), ...lines.flush()];
        assert.deepStrictEqual(
            events.map((e) => e.event),
            ["test", "result", "summary"],
        );
    });

    it("the tests of a list", () => {
        assert.deepStrictEqual(listedTests(`${TEST}\n{"event":"test","file":"b_test.bas","name":"test_sub","line":9}\n`), [
            { file: "C:\\ws\\lib\\math_test.bas", name: "TestAdd", line: 5 },
            { file: "b_test.bas", name: "test_sub", line: 9 },
        ]);
    });
});

describe("testOutput: a run's arguments", () => {
    it("whole files: no filter", () => {
        assert.deepStrictEqual(runArgs([{ file: "a_test.bas" }, { file: "b_test.bas" }]), ["test", "--format", "json", "a_test.bas", "b_test.bas"]);
    });

    it("single tests: their files once, a filter each", () => {
        assert.deepStrictEqual(runArgs([{ file: "a_test.bas", name: "TestOne" }, { file: "a_test.bas", name: "TestTwo" }]), [
            "test",
            "--format",
            "json",
            "a_test.bas",
            "--filter",
            "a_test.bas:TestOne",
            "--filter",
            "a_test.bas:TestTwo",
        ]);
    });

    it("a whole file beside a single test: the file's filter", () => {
        assert.deepStrictEqual(runArgs([{ file: "a_test.bas" }, { file: "b_test.bas", name: "TestB" }]).slice(5), [
            "--filter",
            "a_test.bas:",
            "--filter",
            "b_test.bas:TestB",
        ]);
    });
});

describe("testOutput: states and messages", () => {
    it("errored for an error, a timeout and a build error", () => {
        assert.deepStrictEqual(
            (["pass", "fail", "error", "timeout", "build-error"] as const).map(stateOf),
            ["passed", "failed", "errored", "errored", "errored"],
        );
    });

    it("a build error says so", () => {
        const r: TestResultEvent = { file: "a", name: "T", line: 1, outcome: "build-error", message: "a_test.bas:4:9: error: x", location: null, output: "", ms: 0 };
        assert.strictEqual(messageOf(r), "The test file did not build:\na_test.bas:4:9: error: x");
        assert.strictEqual(messageOf({ ...r, outcome: "timeout", message: "timed out after 30 s" }), "timed out after 30 s");
    });
});
