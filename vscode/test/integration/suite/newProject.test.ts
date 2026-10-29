// "QB64: New Project" (spec editor/new-project), with the folder dialog stubbed: the folder is given to the command.
import * as assert from "assert";
import { execFileSync } from "child_process";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { NewProjectResult } from "../../../src/newProject";
import { closeAll, server, workspace } from "./helpers";

function freshFolder(name: string): string {
    const folder = path.join(workspace, name);
    fs.rmSync(folder, { recursive: true, force: true });
    fs.mkdirSync(folder, { recursive: true });
    return folder;
}

async function newProject(folder: string): Promise<NewProjectResult> {
    return vscode.commands.executeCommand<NewProjectResult>("qb64rust.newProject", vscode.Uri.file(folder));
}

describe("new project", () => {
    after(closeAll);

    it("writes main.bas and qb64rust.toml into an empty folder and opens main.bas", async () => {
        const folder = freshFolder("new-project-empty");
        const result = await newProject(folder);
        const main = path.join(folder, "main.bas");
        // The command's path comes from a `vscode.Uri`, whose drive letter is lower case.
        assert.ok(result && "main" in result, JSON.stringify(result));
        assert.strictEqual(result.main.toLowerCase(), main.toLowerCase());
        assert.ok(fs.readFileSync(main, "utf8").startsWith("PRINT "));
        const toml = fs.readFileSync(path.join(folder, "qb64rust.toml"), "utf8");
        assert.match(toml, /^\[fixes\]\nall = true$/m);
        const open = vscode.window.activeTextEditor?.document.uri.fsPath;
        assert.strictEqual(open?.toLowerCase(), main.toLowerCase());
    });

    it("writes nothing into a folder that holds main.bas, and names it", async () => {
        const folder = freshFolder("new-project-taken");
        fs.writeFileSync(path.join(folder, "main.bas"), "PRINT 1\r\n");
        const result = await newProject(folder);
        assert.ok(result && "message" in result && result.message.includes("main.bas"), JSON.stringify(result));
        assert.strictEqual(fs.readFileSync(path.join(folder, "main.bas"), "utf8"), "PRINT 1\r\n");
        assert.ok(!fs.existsSync(path.join(folder, "qb64rust.toml")));
    });

    it("the new main.bas is built with every fix on", async function () {
        if (!server) {
            this.skip();
        }
        const folder = freshFolder("new-project-fixes");
        await newProject(folder);
        // The compiler's IR dump names the fixes it builds with; a build with "QB64: Build" reads the same file.
        const out = execFileSync(server, ["--dump", "ir", "main.bas"], { cwd: folder, encoding: "utf8" });
        const settings = out.split(/\r?\n/)[0];
        assert.match(settings, /fixes resume-after-block .* checking-on-service bit-number-check$/);
    });
});
