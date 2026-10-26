import * as assert from "assert";
import { Facts, LaunchConfig, initialConfiguration, resolveLaunch, splitArgs } from "../../src/debugConfig";

// Change m5-debug-launch, design D3: what a launch configuration becomes. Made-up paths (tools\repo_check allows
// `does\not\exist`).
const dir = "C:\\does\\not\\exist";

const facts = (more: Partial<Facts> = {}): Facts => ({
    trusted: true,
    activeFile: `${dir}\\hello.bas`,
    folder: dir,
    runArguments: "",
    noPrompt: false,
    buildWith: "auto",
    binary: { found: true },
    ...more,
});

const config = (more: Partial<LaunchConfig> = {}): LaunchConfig => ({ type: "qb64rust", request: "launch", name: "t", ...more });

describe("debug launch resolution", () => {
    it("an untrusted workspace is refused", () => {
        const r = resolveLaunch(config({ program: `${dir}\\hello.bas` }), facts({ trusted: false }));
        assert.strictEqual(r.kind, "refuse");
        assert.match(r.kind === "refuse" ? r.message : "", /trusted workspace/);
    });

    it("no program and no active QB64 file is refused", () => {
        const r = resolveLaunch(config(), facts({ activeFile: undefined }));
        assert.deepStrictEqual(r, { kind: "refuse", message: "open a QB64 program first." });
    });

    it("an executable is launched as it is, whatever the compiler setting", () => {
        const r = resolveLaunch(config({ program: `${dir}\\HELLO.EXE` }), facts({ buildWith: "qb64pe" }));
        assert.strictEqual(r.kind, "launch");
        assert.strictEqual(r.kind === "launch" && r.config.program, `${dir}\\HELLO.EXE`);
    });

    it("a source is built first", () => {
        const r = resolveLaunch(config({ program: `${dir}\\w\\hello.bas` }), facts());
        assert.strictEqual(r.kind, "build");
        assert.strictEqual(r.kind === "build" && r.source, `${dir}\\w\\hello.bas`);
    });

    it("a source with qb64rust.buildWith qb64pe is refused, naming the setting", () => {
        const r = resolveLaunch(config({ program: `${dir}\\hello.bas` }), facts({ buildWith: "qb64pe" }));
        assert.strictEqual(r.kind, "refuse");
        assert.strictEqual(r.kind === "refuse" && r.setting, "qb64rust.buildWith");
    });

    it("a source without the qb64rust binary is refused, naming qb64rust.path, under auto as under qb64rust", () => {
        for (const buildWith of ["auto", "qb64rust"] as const) {
            const r = resolveLaunch(config({ program: `${dir}\\hello.bas` }), facts({ buildWith, binary: { found: false, reason: "not found" } }));
            assert.strictEqual(r.kind, "refuse");
            assert.strictEqual(r.kind === "refuse" && r.setting, "qb64rust.path");
            assert.match(r.kind === "refuse" ? r.message : "", /not found/);
        }
    });

    it("the defaults: the active file, its folder, the integrated terminal, the settings, no stop on entry", () => {
        const r = resolveLaunch(config(), facts({ runArguments: 'a "b c"', noPrompt: true }));
        assert.strictEqual(r.kind, "build");
        const c = r.kind === "build" ? r.config : config();
        assert.strictEqual(c.program, `${dir}\\hello.bas`);
        assert.strictEqual(c.cwd, dir);
        assert.strictEqual(c.console, "integratedTerminal");
        assert.deepStrictEqual(c.args, ["a", "b c"]);
        assert.strictEqual(c.noPrompt, true);
        assert.strictEqual(c.stopOnEntry, false);
    });

    it("what the configuration says wins over the defaults", () => {
        const given = config({ program: `${dir}\\hello.bas`, args: ["x"], cwd: `${dir}\\run`, console: "internalConsole", noPrompt: false, stopOnEntry: true });
        const r = resolveLaunch(given, facts({ runArguments: "a b", noPrompt: true }));
        assert.strictEqual(r.kind, "build");
        assert.deepStrictEqual(r.kind === "build" && r.config, given);
    });

    it("a relative program is resolved against the workspace folder", () => {
        const r = resolveLaunch(config({ program: "sub\\p.bas" }), facts());
        assert.strictEqual(r.kind === "build" && r.source, `${dir}\\sub\\p.bas`);
    });

    it("the initial configuration debugs the active file", () => {
        assert.deepStrictEqual(initialConfiguration(), { type: "qb64rust", request: "launch", name: "Debug QB64 program", program: "${file}" });
    });
});

// The ordinary cases, words and double-quoted words, are where Run (which hands `qb64rust.runArguments` to a shell
// as typed) and a debug session give the program the same arguments.
describe("splitArgs", () => {
    it("words and quoted words", () => {
        assert.deepStrictEqual(splitArgs("a b"), ["a", "b"]);
        assert.deepStrictEqual(splitArgs('  a   "b c"  '), ["a", "b c"]);
        assert.deepStrictEqual(splitArgs('"a"b "" c'), ["ab", "", "c"]);
    });

    it("empty text gives no argument", () => {
        assert.deepStrictEqual(splitArgs(""), []);
        assert.deepStrictEqual(splitArgs("   "), []);
    });

    it("backslashes and quotes as the C runtime reads them", () => {
        assert.deepStrictEqual(splitArgs('a\\"b'), ['a"b']);
        assert.deepStrictEqual(splitArgs('"a\\\\" b'), ["a\\", "b"]);
        assert.deepStrictEqual(splitArgs(`${dir}\\file x`), [`${dir}\\file`, "x"]);
        assert.deepStrictEqual(splitArgs('"say ""hi"""'), ['say "hi"']);
    });
});
