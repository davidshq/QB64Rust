import * as assert from "assert";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import { ensureExample, exampleFolder, resetExample } from "../../src/examples/exampleCopy";
import { HelpExample, HelpIndex } from "../../src/help/helpIndex";

describe("the user's copy of an example (spec editor/examples, \"An example opens as a copy\")", () => {
    let dir: string;
    let storage: string;
    let index: HelpIndex;
    const shipped = () => path.join(dir, "help", "examples", "first-steps", "hello.bas");

    beforeEach(() => {
        dir = fs.mkdtempSync(path.join(os.tmpdir(), "qb64rust-example-"));
        storage = path.join(dir, "storage");
        const examples = path.join(dir, "help", "examples");
        fs.mkdirSync(path.join(examples, "first-steps"), { recursive: true });
        fs.writeFileSync(shipped(), 'PRINT "Hello"\r\n');
        fs.mkdirSync(path.join(examples, "open_1.files", "data"), { recursive: true });
        fs.writeFileSync(path.join(examples, "open_1.bas"), 'OPEN "data/in.txt" FOR INPUT AS #1\r\n');
        fs.writeFileSync(path.join(examples, "open_1.files", "data", "in.txt"), "one\r\n");
        const example = (name: string, files: boolean): HelpExample => ({ name, title: name, file: `examples/${name}.bas`, output: false, files, network: false });
        const hello = example("first-steps/hello", false);
        index = new HelpIndex(path.join(dir, "help"), {
            pages: [],
            tree: [],
            groups: [{ id: "first-steps", title: "First steps", examples: [hello] }],
            keywords: [{ page: "open", title: "OPEN", examples: [example("open_1", true)] }],
        });
    });
    afterEach(() => fs.rmSync(dir, { recursive: true, force: true }));

    it("the first click makes a file in the storage, and the shipped file is not the one opened", () => {
        const copy = ensureExample(index, "first-steps/hello", storage);
        assert.strictEqual(copy, path.join(exampleFolder(storage, "first-steps/hello"), "hello.bas"));
        assert.notStrictEqual(copy, shipped());
        assert.strictEqual(fs.readFileSync(copy!, "utf8"), 'PRINT "Hello"\r\n');
        assert.strictEqual(ensureExample(index, "no-such", storage), undefined);
    });

    it("an edited copy is kept, and Reset gives the shipped text back", () => {
        const copy = ensureExample(index, "first-steps/hello", storage)!;
        fs.writeFileSync(copy, 'PRINT "Mine"\r\n');
        assert.strictEqual(ensureExample(index, "first-steps/hello", storage), copy);
        assert.strictEqual(fs.readFileSync(copy, "utf8"), 'PRINT "Mine"\r\n');
        // The shipped file was not touched.
        assert.strictEqual(fs.readFileSync(shipped(), "utf8"), 'PRINT "Hello"\r\n');
        assert.strictEqual(resetExample(index, "first-steps/hello", storage), copy);
        assert.strictEqual(fs.readFileSync(copy, "utf8"), 'PRINT "Hello"\r\n');
    });

    it("an untouched copy follows a new version of the example; an edited one does not", () => {
        const copy = ensureExample(index, "first-steps/hello", storage)!;
        fs.writeFileSync(shipped(), 'PRINT "Hello, again"\r\n');
        ensureExample(index, "first-steps/hello", storage);
        assert.strictEqual(fs.readFileSync(copy, "utf8"), 'PRINT "Hello, again"\r\n');
        fs.writeFileSync(copy, 'PRINT "Mine"\r\n');
        fs.writeFileSync(shipped(), 'PRINT "Hello, once more"\r\n');
        ensureExample(index, "first-steps/hello", storage);
        assert.strictEqual(fs.readFileSync(copy, "utf8"), 'PRINT "Mine"\r\n');
    });

    it("the contents of the example's .files folder are beside the copy", () => {
        const copy = ensureExample(index, "open_1", storage)!;
        assert.strictEqual(fs.readFileSync(path.join(path.dirname(copy), "data", "in.txt"), "utf8"), "one\r\n");
    });
});
