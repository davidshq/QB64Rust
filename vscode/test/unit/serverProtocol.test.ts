import * as assert from "assert";
import { DOCUMENT_ENCODING, EncodingTracker, initializationOptions } from "../../src/server/protocol";

describe("language server protocol", () => {
    it("names the server's encoding notification", () => {
        assert.strictEqual(DOCUMENT_ENCODING, "qb64rust/documentEncoding");
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
