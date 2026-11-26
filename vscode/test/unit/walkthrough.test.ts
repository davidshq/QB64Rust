import * as assert from "assert";
import * as fs from "fs";
import * as path from "path";

const root = path.resolve(__dirname, "../../..");
const manifest = JSON.parse(fs.readFileSync(path.join(root, "package.json"), "utf8"));

interface Step {
    id: string;
    title: string;
    description: string;
    media: { markdown: string };
    completionEvents: string[];
    when?: string;
}

describe("the walkthrough Get Started with QB64 (spec editor/walkthrough)", () => {
    const walkthrough = manifest.contributes.walkthroughs[0];
    const steps = walkthrough.steps as Step[];
    const contributed = new Set((manifest.contributes.commands as { command: string }[]).map((c) => c.command));

    it("its steps, in order", () => {
        assert.strictEqual(walkthrough.title, "Get Started with QB64");
        assert.deepStrictEqual(
            steps.map((s) => s.title),
            [
                "Write your first program",
                "Run it",
                "Run it",
                "Look a word up",
                "Try an example",
                "Find and fix a mistake",
                "Put your program on the web (optional)",
            ],
        );
    });

    it("every button names a command the extension contributes, and ticks its step", () => {
        for (const s of steps) {
            const links = [...s.description.matchAll(/\(command:([\w.]+)/g)].map((m) => m[1]);
            assert.strictEqual(links.length, 1, `${s.id}: one button`);
            assert.ok(contributed.has(links[0]), `${s.id}: ${links[0]} is not contributed`);
            for (const e of s.completionEvents) {
                const command = /^onCommand:(.+)$/.exec(e)?.[1];
                assert.ok(command === undefined || contributed.has(command), `${s.id}: ${e}`);
            }
            // A context key the step's work sets: `onCommand:` misses a command the extension runs itself (task 6.2).
            assert.ok(
                s.completionEvents.some((e) => e.startsWith("onContext:qb64rust.walkthrough.")),
                `${s.id}: no context key`,
            );
            assert.ok(fs.existsSync(path.join(root, s.media.markdown)), `${s.id}: no ${s.media.markdown}`);
        }
    });

    it("the run step tells of the download first; on Linux, of g++", () => {
        const windows = fs.readFileSync(path.join(root, "walkthrough", "run-windows.md"), "utf8");
        assert.ok(windows.includes("190 MB") && windows.includes("github.com/mstorsjo/llvm-mingw") && windows.includes("unless you agree"));
        const linux = fs.readFileSync(path.join(root, "walkthrough", "run-linux.md"), "utf8");
        assert.ok(linux.includes("`g++`"));
        assert.deepStrictEqual(
            steps.filter((s) => s.title === "Run it").map((s) => s.when),
            ["!isLinux", "isLinux"],
        );
    });

    it("QB64: Get Started is a command of the extension", () => {
        assert.ok(contributed.has("qb64rust.getStarted"));
    });
});
