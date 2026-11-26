import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";
import { EVERY_KEYWORD, exampleContext, exampleTree } from "../../src/examples/examplesModel";
import { HelpExample, HelpIndexData, wordAt } from "../../src/help/helpIndex";

const manifest = JSON.parse(fs.readFileSync(path.resolve(__dirname, "../../../package.json"), "utf8"));

function example(name: string, title = name, network = false): HelpExample {
    return { name, title, file: `examples/${name}.bas`, output: false, files: false, network };
}

const DATA: HelpIndexData = {
    pages: [],
    tree: [],
    groups: [
        { id: "first-steps", title: "First steps", examples: [example("first-steps/hello", "Hello"), example("first-steps/guess", "Guess the number")] },
        { id: "graphics", title: "Graphics", examples: [example("graphics/bounce", "Bouncing ball")] },
        { id: "sound", title: "Sound and music", examples: [] },
        { id: "games", title: "Small games", examples: [] },
    ],
    keywords: [
        { page: "_clamp", title: "_CLAMP", examples: [example("_clamp_1", "_CLAMP")] },
        { page: "_openhost", title: "_OPENHOST", examples: [example("_openhost_1", "_OPENHOST (1)", true), example("_openhost_2", "_OPENHOST (2)")] },
    ],
};

describe("the Examples view (spec editor/examples)", () => {
    it("the four groups, then one for every keyword, and Hello in First steps", () => {
        const tree = exampleTree(DATA);
        assert.deepStrictEqual(
            tree.map((r) => r.label),
            ["First steps", "Graphics", "Sound and music", "Small games", EVERY_KEYWORD],
        );
        const first = tree[0];
        assert.ok(first.kind === "group");
        assert.deepStrictEqual(
            first.children.map((r) => r.label),
            ["Hello", "Guess the number"],
        );
        // "First steps" starts open; the other groups start closed.
        assert.deepStrictEqual(
            tree.map((r) => r.kind === "group" && Boolean(r.open)),
            [true, false, false, false, false],
        );
        const every = tree[4];
        assert.ok(every.kind === "group");
        // One example is a row; several get a row of their own.
        assert.strictEqual(every.children[0].kind, "example");
        assert.strictEqual(every.children[1].kind, "group");
    });

    it("an example that opens a port gets no Run", () => {
        assert.strictEqual(exampleContext(example("_openhost_1", "x", true)), "example.network");
        assert.strictEqual(exampleContext(example("_clamp_1")), "example.runnable");
        // The manifest shows Run only on a runnable row.
        const runs = manifest.contributes.menus["view/item/context"].filter((m: { command: string }) => m.command === "qb64rust.examples.run");
        assert.ok(runs.length > 0);
        for (const m of runs) {
            assert.ok(m.when.includes("viewItem == example.runnable"), m.when);
        }
    });
});

describe("help for the word under the cursor (spec editor/help)", () => {
    it("the word at a column, with its suffix", () => {
        const line = "health = _CLAMP(x$, 0, 100) ' LEFT$";
        assert.deepStrictEqual(wordAt(line, 11), { word: "_CLAMP", start: 9 });
        assert.deepStrictEqual(wordAt(line, 15), { word: "_CLAMP", start: 9 });
        assert.deepStrictEqual(wordAt(line, 17), { word: "x$", start: 16 });
        assert.strictEqual(wordAt(line, 21), undefined);
        assert.deepStrictEqual(wordAt("$CONSOLE:ONLY", 2), { word: "$CONSOLE", start: 0 });
    });

    it("no key is bound: F1 stays VS Code's (user, 2026-11-25)", () => {
        assert.strictEqual(manifest.contributes.keybindings, undefined);
    });

    it("the command is in the editor's right-click menu for QB64 files", () => {
        const menu = manifest.contributes.menus["editor/context"] as { command: string; when: string }[];
        assert.ok(menu.some((m) => m.command === "qb64rust.helpForWord" && m.when === "editorLangId == qb64rust"));
    });
});
