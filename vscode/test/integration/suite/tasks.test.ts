import * as assert from "assert";
import * as fs from "fs";
import * as vscode from "vscode";
import { closeAll, compiler, open, writeFile } from "./helpers";

describe("tasks", function () {
    this.timeout(600000);

    before(function () {
        if (!compiler) {
            this.skip();
        }
    });
    after(closeAll);

    it("provides build and run tasks; the build task compiles the active file", async () => {
        const uri = writeFile("tasks/prog.bas", '$CONSOLE:ONLY\r\nPRINT "task"\r\n');
        const exe = uri.fsPath.replace(/\.bas$/i, ".exe");
        await open(uri);
        const tasks = await vscode.tasks.fetchTasks({ type: "qb64rust" });
        assert.deepStrictEqual(tasks.map((t) => t.name).sort(), ["build", "run"]);
        const build = tasks.find((t) => t.name === "build")!;
        const ended = new Promise<number | undefined>((resolve) => {
            const sub = vscode.tasks.onDidEndTaskProcess((e) => {
                if (e.execution.task.name === "build" && e.execution.task.source === "qb64rust") {
                    sub.dispose();
                    resolve(e.exitCode);
                }
            });
        });
        await vscode.tasks.executeTask(build);
        assert.strictEqual(await ended, 0);
        assert.ok(fs.existsSync(exe));
    });
});
