import * as assert from "assert";
import { CHECK, DOCUMENT_ENCODING, EncodingTracker, checkParams, includeRootFor, initializationOptions } from "../../src/server/protocol";

describe("language server protocol", () => {
    it("names the server's encoding notification", () => {
        assert.strictEqual(DOCUMENT_ENCODING, "qb64rust/documentEncoding");
    });

    it("names the check request, whose parameter is the document's uri", () => {
        assert.strictEqual(CHECK, "qb64rust/check");
        assert.deepStrictEqual(checkParams("file:///c%3A/p/a.bas"), { uri: "file:///c%3A/p/a.bas" });
    });

    it("initialization options: the default encoding, cp437 when none is set", () => {
        assert.deepStrictEqual(initializationOptions("utf8", "", undefined, "linux"), { encoding: "utf8", includeRoot: "" });
        assert.deepStrictEqual(initializationOptions(undefined, "", undefined, "linux"), { encoding: "cp437", includeRoot: "" });
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
