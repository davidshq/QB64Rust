import * as assert from "assert";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import { dialogFolder, exampleStem, folderIsUtf8, freeName, tomlProjectEncoding, withBas } from "../../src/newProgramPlan";

describe("QB64: New Program (spec editor/new-program)", () => {
    let dir: string;
    beforeEach(() => {
        dir = fs.mkdtempSync(path.join(os.tmpdir(), "qb64rust-newprogram-"));
    });
    afterEach(() => fs.rmSync(dir, { recursive: true, force: true }));

    it("the free name: program1.bas, then the next the folder does not hold", () => {
        assert.strictEqual(freeName(dir), "program1.bas");
        fs.writeFileSync(path.join(dir, "program1.bas"), "");
        assert.strictEqual(freeName(dir), "program2.bas");
        assert.strictEqual(freeName(dir, "clamp"), "clamp.bas");
        fs.writeFileSync(path.join(dir, "clamp.bas"), "");
        assert.strictEqual(freeName(dir, "clamp"), "clamp2.bas");
    });

    it("an example's name", () => {
        assert.strictEqual(exampleStem("_clamp_1"), "clamp");
        assert.strictEqual(exampleStem("first-steps/hello"), "hello");
        assert.strictEqual(exampleStem("left-str_2"), "left");
        assert.strictEqual(exampleStem("meta-include_1"), "include");
    });

    it("a name without .bas gets it", () => {
        assert.strictEqual(withBas(path.join(dir, "bounce")), path.join(dir, "bounce.bas"));
        assert.strictEqual(withBas("a.BAS"), "a.BAS");
        assert.strictEqual(withBas("lib.bm"), "lib.bm");
    });

    describe("where the dialog opens", () => {
        const exists = (have: string[]) => (f: string) => have.includes(f);
        const home = path.join(os.tmpdir(), "qb64rust-no-such-home");

        it("a workspace folder; with several, the one of the active file", () => {
            assert.strictEqual(dialogFolder({ workspaceFolders: ["/w/games"], home, exists: exists([]) }), "/w/games");
            const two = ["/w/a", "/w/b"];
            assert.strictEqual(dialogFolder({ workspaceFolders: two, activeFileFolder: "/w/b/src", home, exists: exists([]) }), "/w/b");
            assert.strictEqual(dialogFolder({ workspaceFolders: two, activeFileFolder: "/elsewhere", home, exists: exists([]) }), "/w/a");
        });

        it("no folder: the folder last used, while it is there", () => {
            const last = path.join("D:", "basic");
            assert.strictEqual(dialogFolder({ workspaceFolders: [], lastFolder: last, home, exists: exists([last]) }), last);
        });

        it("no folder: one that is gone gives the documents folder, else the home", () => {
            const documents = path.join(home, "Documents");
            assert.strictEqual(dialogFolder({ workspaceFolders: [], lastFolder: "/gone", home, exists: exists([documents]) }), documents);
            assert.strictEqual(dialogFolder({ workspaceFolders: [], home, exists: exists([documents]) }), documents);
            assert.strictEqual(dialogFolder({ workspaceFolders: [], home, exists: exists([]) }), home);
        });
    });

    it("the folder's encoding from the nearest qb64rust.toml", () => {
        assert.strictEqual(tomlProjectEncoding('[fixes]\nall = true\n\n[project]\nencoding = "utf-8"\n'), "utf-8");
        assert.strictEqual(tomlProjectEncoding('[build]\nencoding = "utf-8"\n'), undefined);
        assert.strictEqual(tomlProjectEncoding("[project] # x\nencoding = 'cp437' # y\n"), "cp437");
        const sub = path.join(dir, "games", "new");
        fs.mkdirSync(sub, { recursive: true });
        assert.strictEqual(folderIsUtf8(sub), false);
        fs.writeFileSync(path.join(dir, "games", "qb64rust.toml"), '[project]\nencoding = "utf-8"\n');
        assert.strictEqual(folderIsUtf8(sub), true);
        // The nearest one wins.
        fs.writeFileSync(path.join(sub, "qb64rust.toml"), "[fixes]\nall = true\n");
        assert.strictEqual(folderIsUtf8(sub), false);
    });
});
