import * as assert from "assert";
import { InitPlan, initPlanArgs, initWriteArgs, parseInitPlan } from "../../src/compiler/qb64rust";
import { answersOf, copyQuestion, startOf } from "../../src/setUpPlan";

// Change project-init, design D8: what "QB64: Set Up Project" asks from `qb64rust init --plan`, and the command lines
// it runs. Made-up paths (tools\repo_check allows `does\not\exist`).
const dir = "C:\\does\\not\\exist";

const plan = (over: Partial<InitPlan> = {}): InitPlan => ({
    folder: `${dir}\\w`,
    settings: null,
    newProject: false,
    programs: ["game.bas"],
    dataFiles: [],
    git: false,
    ...over,
});

describe("qb64rust init from the extension", () => {
    it("the plan's line of JSON, and output that is not JSON (an older qb64rust)", () => {
        const line = JSON.stringify({
            folder: `${dir}\\w`,
            settings: null,
            above: null,
            new_project: false,
            programs: ["game.bas"],
            data_files: ["GAME.CFG", "maps"],
            git: true,
            main: "game.bas",
            fixes: false,
        });
        assert.deepStrictEqual(parseInitPlan(`${line}\r\n`), plan({ dataFiles: ["GAME.CFG", "maps"], git: true }));
        assert.strictEqual(parseInitPlan("qb64rust: cannot read init\r\n"), undefined);
        assert.strictEqual(parseInitPlan('{"exe":"x"}'), undefined);
        assert.deepStrictEqual(initPlanArgs(`${dir}\\w`), ["init", `${dir}\\w`, "--plan"]);
    });

    it("the arguments of each answer", () => {
        const w = `${dir}\\w`;
        assert.deepStrictEqual(initWriteArgs(w, { output: ".", copy: ["ignored"] }), ["init", w, "--yes", "--output", "."]);
        assert.deepStrictEqual(initWriteArgs(w, { output: "build", copy: [] }), ["init", w, "--yes", "--output", "build", "--no-copy"]);
        assert.deepStrictEqual(initWriteArgs(w, { main: "game.bas", output: "build", copy: ["GAME.CFG", "maps"] }), [
            "init",
            w,
            "--yes",
            "--output",
            "build",
            "--main",
            "game.bas",
            "--copy",
            "GAME.CFG",
            "--copy",
            "maps",
        ]);
    });

    it("one program: only where the program goes, beside the source first", () => {
        const start = startOf(plan());
        assert.strictEqual(start.kind, "ask");
        const questions = start.kind === "ask" ? start.questions : [];
        assert.deepStrictEqual(questions.map((q) => q.key), ["output"]);
        assert.deepStrictEqual(questions[0].choices.map((c) => c.value), [".", "build"]);
    });

    it("several programs: the main file first, the first by name the default", () => {
        const start = startOf(plan({ programs: ["editor.bas", "game.bas"] }));
        const questions = start.kind === "ask" ? start.questions : [];
        assert.deepStrictEqual(questions.map((q) => q.key), ["main", "output"]);
        assert.deepStrictEqual(questions[0].choices.map((c) => c.value), ["editor.bas", "game.bas"]);
    });

    it("a build folder with data files: every one ticked; none, or beside the source: no question", () => {
        const q = copyQuestion(plan({ dataFiles: ["GAME.CFG", "maps"] }), "build");
        assert.ok(q && q.many);
        assert.deepStrictEqual(q?.choices.map((c) => c.value), ["GAME.CFG", "maps"]);
        assert.strictEqual(copyQuestion(plan(), "build"), undefined);
        assert.strictEqual(copyQuestion(plan({ dataFiles: ["GAME.CFG"] }), "."), undefined);
        assert.deepStrictEqual(answersOf({ output: ["build"], copy: ["GAME.CFG"] }), { output: "build", copy: ["GAME.CFG"] });
        assert.deepStrictEqual(answersOf({ main: ["game.bas"], output: ["."] }), { main: "game.bas", output: ".", copy: [] });
    });

    it("a settings file there, a new project, and a git work tree", () => {
        assert.deepStrictEqual(startOf(plan({ settings: `${dir}\\w\\qb64rust.toml` })), { kind: "open", settings: `${dir}\\w\\qb64rust.toml` });
        assert.deepStrictEqual(startOf(plan({ programs: [], newProject: true })), { kind: "newProject" });
        // A work tree asks nothing more: the compiler prints the `.gitignore` line, the extension asks no question.
        assert.deepStrictEqual(startOf(plan({ git: true })), startOf(plan()));
    });
});
