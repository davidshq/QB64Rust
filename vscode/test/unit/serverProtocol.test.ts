import * as assert from "assert";
import {
    CHECK,
    DEFAULT_FORMAT,
    DOCUMENT_ENCODING,
    EncodingTracker,
    PROGRAM,
    ProgramTracker,
    checkParams,
    configurationParams,
    formatFlags,
    includeRootFor,
    initializationOptions,
    programStatus,
} from "../../src/server/protocol";

describe("language server protocol", () => {
    it("names the server's encoding notification", () => {
        assert.strictEqual(DOCUMENT_ENCODING, "qb64rust/documentEncoding");
    });

    it("names the check request, whose parameter is the document's uri", () => {
        assert.strictEqual(CHECK, "qb64rust/check");
        assert.deepStrictEqual(checkParams("file:///c%3A/p/a.bas"), { uri: "file:///c%3A/p/a.bas" });
    });

    it("initialization options: the default encoding, cp437 when none is set", () => {
        assert.deepStrictEqual(initializationOptions("utf8", "", undefined, "linux"), { encoding: "utf8", includeRoot: "", format: DEFAULT_FORMAT });
        assert.deepStrictEqual(initializationOptions(undefined, "", undefined, "linux"), { encoding: "cp437", includeRoot: "", format: DEFAULT_FORMAT });
    });

    // Change m5b-formatter, design D7: the layout settings, sent at the start and on a change.
    it("initialization options: the layout settings, QB64pe's defaults unless given", () => {
        assert.deepStrictEqual(DEFAULT_FORMAT, { autoIndent: true, indentSize: 4, indentSubs: true, autoLayout: true, keywordCase: "camel" });
        const format = { ...DEFAULT_FORMAT, indentSize: 2, keywordCase: "lower" as const };
        assert.deepStrictEqual(initializationOptions("cp437", "", undefined, "linux", undefined, format).format, format);
        assert.deepStrictEqual(configurationParams(format), { settings: { format } });
    });

    it("-f: settings: all six, the keyword case as QB64pe's two flags", () => {
        assert.deepStrictEqual(formatFlags(DEFAULT_FORMAT), [
            "-f:autoindent=1",
            "-f:autoindentsize=4",
            "-f:indentsubs=1",
            "-f:autolayout=1",
            "-f:keywordcapitals=0",
            "-f:keywordlowercase=0",
        ]);
        const off = { autoIndent: false, indentSize: 64, indentSubs: false, autoLayout: false, keywordCase: "upper" as const };
        assert.deepStrictEqual(formatFlags(off), [
            "-f:autoindent=0",
            "-f:autoindentsize=64",
            "-f:indentsubs=0",
            "-f:autolayout=0",
            "-f:keywordcapitals=1",
            "-f:keywordlowercase=0",
        ]);
        assert.deepStrictEqual(formatFlags({ ...DEFAULT_FORMAT, keywordCase: "lower" }).slice(-2), ["-f:keywordcapitals=0", "-f:keywordlowercase=1"]);
    });

    it("initialization options: a relative include root is resolved against the workspace folder", () => {
        const o = initializationOptions("cp437", "lib/qb", "/does/not/exist", "linux");
        assert.strictEqual(o.includeRoot, "/does/not/exist/lib/qb");
        assert.strictEqual(initializationOptions("cp437", " /opt/qb ", "/does/not/exist", "linux").includeRoot, "/opt/qb");
        assert.strictEqual(initializationOptions("cp437", " /opt/qb ", undefined, "linux").includeRoot, "/opt/qb");
    });

    it("initialization options: a relative include root without a workspace folder gives the server's default", () => {
        assert.strictEqual(initializationOptions("cp437", "lib", undefined, "linux").includeRoot, "");
    });

    // Spec editor/language-server, "Included files" (design D6 of extension-vsix).
    it("include root: the setting, else the QB64pe folder, else the server's default", () => {
        assert.strictEqual(includeRootFor("/opt/inc", undefined, "/opt/qb64pe", "linux"), "/opt/inc");
        assert.strictEqual(includeRootFor("", undefined, "/opt/qb64pe", "linux"), "/opt/qb64pe");
        assert.strictEqual(includeRootFor("lib", undefined, "/opt/qb64pe", "linux"), "/opt/qb64pe");
        assert.strictEqual(includeRootFor("", undefined, undefined, "linux"), "");
        assert.strictEqual(initializationOptions("cp437", "", undefined, "win32", "C:\\qb64pe").includeRoot, "C:\\qb64pe");
    });

    // Change include-file-program, design D8: spec editor/language-server, "Program status item".
    it("names the program notification, and shows the program's file name", () => {
        assert.strictEqual(PROGRAM, "qb64rust/program");
        const turn = { uri: "file:///c%3A/cws/rules/turn.bm", program: "file:///c%3A/cws/CWSTRAT.BAS", source: "disk" as const };
        assert.strictEqual(programStatus(turn)?.text, "CWSTRAT.BAS");
        assert.match(programStatus(turn)?.detail ?? "", /on disk/);
        const named = { uri: "file:///c%3A/p/lib.bm", program: "file:///c%3A/p/my%20game.bas", source: "settings" as const };
        assert.strictEqual(programStatus(named)?.text, "my game.bas");
        // "A .bas file inside another program".
        const other = { uri: "file:///c%3A/p/other.bas", program: "file:///c%3A/p/main.bas", source: "open" as const };
        assert.strictEqual(programStatus(other)?.text, "main.bas");
    });

    it("shows no program found for an include file alone, and nothing for a .bas file of its own or before the server said", () => {
        assert.strictEqual(programStatus({ uri: "file:///c%3A/p/lib.bm", program: null, source: "none" })?.text, "no program found");
        assert.strictEqual(programStatus({ uri: "file:///c%3A/p/main.bas", program: null, source: "none" }), undefined);
        assert.strictEqual(programStatus({ uri: "file:///c%3A/p/MAIN.BAS", program: null, source: "none" }), undefined);
        assert.strictEqual(programStatus({ uri: "untitled:Untitled-1", program: null, source: "none" }), undefined);
        assert.strictEqual(programStatus(undefined), undefined);
    });

    it("keeps the last program of each document until it closes", () => {
        const t = new ProgramTracker();
        const lib = "file:///c%3A/p/lib.bm";
        t.update({ uri: lib, program: null, source: "none" });
        t.update({ uri: lib, program: "file:///c%3A/p/main.bas", source: "open" });
        assert.strictEqual(t.get(lib)?.source, "open");
        t.forget(lib);
        assert.strictEqual(t.get(lib), undefined);
    });

    it("tells each document's encoding once, and again after a change or a close", () => {
        const t = new EncodingTracker();
        assert.deepStrictEqual(t.update("file:///a.bas", "cp437"), { uri: "file:///a.bas", encoding: "cp437" });
        assert.strictEqual(t.update("file:///a.bas", "cp437"), undefined);
        assert.deepStrictEqual(t.update("file:///a.bas", "utf8"), { uri: "file:///a.bas", encoding: "utf8" });
        t.forget("file:///a.bas");
        assert.deepStrictEqual(t.update("file:///a.bas", "utf8"), { uri: "file:///a.bas", encoding: "utf8" });
        t.clear();
        assert.notStrictEqual(t.update("file:///a.bas", "utf8"), undefined);
    });
});
