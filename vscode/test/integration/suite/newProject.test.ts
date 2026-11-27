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
        assert.match(toml, /^\[project\]\nencoding = "utf-8"$/m);
        const settings = JSON.parse(fs.readFileSync(path.join(folder, ".vscode", "settings.json"), "utf8"));
        assert.deepStrictEqual(settings, { "[qb64rust]": { "files.encoding": "utf8" } });
        const open = vscode.window.activeTextEditor?.document;
        assert.strictEqual(open?.uri.fsPath.toLowerCase(), main.toLowerCase());
        assert.strictEqual(open?.encoding, "utf8");
    });

    it("writes nothing into a folder that holds .vscode/settings.json, and names it", async () => {
        const folder = freshFolder("new-project-settings-taken");
        fs.mkdirSync(path.join(folder, ".vscode"));
        fs.writeFileSync(path.join(folder, ".vscode", "settings.json"), "{}\n");
        const result = await newProject(folder);
        assert.ok(result && "message" in result && result.message.includes("settings.json"), JSON.stringify(result));
        assert.ok(!fs.existsSync(path.join(folder, "main.bas")));
        assert.strictEqual(fs.readFileSync(path.join(folder, ".vscode", "settings.json"), "utf8"), "{}\n");
    });

    it("a box-drawing character typed into the new main.bas is saved as UTF-8 and built as one character", async function () {
        const folder = freshFolder("new-project-utf8");
        await newProject(folder);
        const editor = vscode.window.activeTextEditor;
        assert.ok(editor);
        const whole = new vscode.Range(0, 0, editor.document.lineCount, 0);
        await editor.edit((e) => e.replace(whole, 'PRINT "─"\r\n'));
        assert.ok(await editor.document.save());
        const main = path.join(folder, "main.bas");
        assert.deepStrictEqual([...fs.readFileSync(main)], [...Buffer.from('PRINT "─"\r\n', "utf8")]);
        if (!server) {
            this.skip();
        }
        // The string holds the one code page 437 byte of `─`, which the program's screen draws as `─`.
        const out = execFileSync(server, ["--dump", "ir", "main.bas"], { cwd: folder, encoding: "utf8" });
        assert.ok(out.includes('Str Const "\\xC4"'), out);
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
