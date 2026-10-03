import * as assert from "assert";
import * as path from "path";
import { RunQueue, RunKind, RunRequest } from "../../src/compiler/runQueue";

const fake = path.resolve(__dirname, "../../../test-fixtures/fake-process.js");

function req(kind: RunKind, label: string, sleepMs: number, exitCode = 0, extra: Partial<RunRequest> = {}): RunRequest {
    return { kind, key: extra.key, command: process.execPath, args: [fake, label, String(sleepMs), String(exitCode)], ...extra };
}

function isAlive(pid: number): boolean {
    try {
        process.kill(pid, 0);
        return true;
    } catch {
        return false;
    }
}

describe("RunQueue", function () {
    this.timeout(20000);

    it("runs one process at a time, in order", async () => {
        const q = new RunQueue();
        const events: string[] = [];
        const p1 = q.run(req("build", "a", 300, 0, { onOutput: (t) => events.push(t.trim()) }));
        const p2 = q.run(req("format", "b", 50, 3, { onOutput: (t) => events.push(t.trim()) }));
        const [r1, r2] = await Promise.all([p1, p2]);
        assert.strictEqual(r1.exitCode, 0);
        assert.strictEqual(r2.exitCode, 3);
        const order = events.join("\n").split("\n").map((l) => l.split(" ").slice(0, 2).join(" "));
        assert.deepStrictEqual(order, ["start a", "end a", "start b", "end b"]);
        assert.ok(q.idle);
    });

    it("a new check of the same file cancels the queued one", async () => {
        const q = new RunQueue();
        const build = q.run(req("build", "build", 300));
        const first = q.run(req("check", "c1", 50, 0, { key: "f.bas" }));
        const second = q.run(req("check", "c2", 50, 0, { key: "f.bas" }));
        const other = q.run(req("check", "c3", 50, 0, { key: "g.bas" }));
        const results = await Promise.all([build, first, second, other]);
        assert.deepStrictEqual(results.map((r) => r.cancelled), [false, true, false, false]);
        assert.match(results[2].output, /end c2/);
        assert.match(results[3].output, /end c3/);
    });

    it("a new check of the same file kills the running one and its child processes", async () => {
        const q = new RunQueue();
        let firstOutput = "";
        const first = q.run(req("check", "slow", 10000, 0, { key: "f.bas", onOutput: (t) => (firstOutput += t) }));
        while (!/grandchild (\d+)/.test(firstOutput)) {
            await new Promise((r) => setTimeout(r, 20));
        }
        const grandchild = Number(/grandchild (\d+)/.exec(firstOutput)![1]);
        const started = Date.now();
        const second = q.run(req("check", "fast", 10, 0, { key: "f.bas" }));
        const r1 = await first;
        assert.ok(r1.cancelled);
        assert.ok(Date.now() - started < 5000);
        assert.ok(!(await second).cancelled);
        await new Promise((r) => setTimeout(r, 300));
        assert.ok(!isAlive(grandchild), "grandchild still running");
    });

    it("checks never cancel a build", async () => {
        const q = new RunQueue();
        const build = q.run(req("build", "b", 300, 0, { key: "f.bas" }));
        const check = q.run(req("check", "c", 10, 0, { key: "f.bas" }));
        assert.ok(!(await build).cancelled);
        assert.ok(!(await check).cancelled);
    });

    it("timeout kills the process", async () => {
        const q = new RunQueue();
        const started = Date.now();
        const r = await q.run(req("check", "hang", 10000, 0, { key: "f.bas", timeoutMs: 500 }));
        assert.ok(r.timedOut);
        assert.ok(Date.now() - started < 5000);
    });

    it("a missing executable gives a failed result, and the queue continues", async () => {
        const q = new RunQueue();
        const bad = await q.run({ kind: "check", command: path.join(__dirname, "no-such-program.exe"), args: [] });
        assert.notStrictEqual(bad.exitCode, 0);
        const good = await q.run(req("check", "ok", 10));
        assert.strictEqual(good.exitCode, 0);
    });

    it("reports busy and idle", async () => {
        const q = new RunQueue();
        const seen: (RunKind | undefined)[] = [];
        q.onBusyChange = (k) => seen.push(k);
        await Promise.all([q.run(req("build", "a", 10)), q.run(req("check", "b", 10, 0, { key: "x" }))]);
        assert.deepStrictEqual(seen, ["build", "check", undefined]);
    });
});
