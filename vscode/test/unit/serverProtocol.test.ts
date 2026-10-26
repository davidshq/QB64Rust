import * as assert from "assert";
import {
    CHECK,
    DEFAULT_FORMAT,
    DOCUMENT_ENCODING,
    EncodingTracker,
    checkParams,
    configurationParams,
    formatFlags,
    includeRootFor,
    initializationOptions,
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
