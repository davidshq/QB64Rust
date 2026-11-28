// "QB64: Set Up Project" and `qb64rust init` (change project-init, tasks 4.3 and 4.4; spec editor/project-setup).
// Needs the qb64rust binary (QB64RUST_TEST_QB64RUST); skipped without it. The lists are answered by the picks given
// to the command, as newProject's test gives the folder in place of the dialog.
import * as assert from "assert";
import { execFileSync } from "child_process";
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import { MAIN_BAS, SETTINGS_TOML, WORKSPACE_SETTINGS } from "../../../src/newProject";
import type { SetUpResult } from "../../../src/setUpProject";
import { api, closeAll, server, workspace, writeFile } from "./helpers";

/** The settings of a settings file without its comments: the plain lines, each without its trailing comment. */
function plainLines(text: string): string[] {
    return text
        .split(/\r?\n/)
        .filter((l) => l.trim() !== "" && !l.startsWith("#"))
        .map((l) => l.split("  #")[0].trim());
}

function setUp(folder: string, picks?: (string[] | undefined)[]): Thenable<SetUpResult> {
    return vscode.commands.executeCommand<SetUpResult>("qb64rust.setUpProject", vscode.Uri.file(folder), picks);
}

describe("set up a project", function () {
    this.timeout(60000);

    before(async function () {
        if (!server) {
            console.log("    skipped: needs QB64RUST_TEST_QB64RUST");
            this.skip();
        }
        await (await api()).server.ready;
    });
    afterEach(closeAll);

    it("a program beside its data: the build folder and the data file ticked are written, and the file opens", async () => {
        const uri = writeFile("setup/data/game.bas", "PRINT 1\r\n");
        writeFile("setup/data/GAME.CFG", "1\r\n");
        const folder = path.dirname(uri.fsPath);
        const result = await setUp(folder, [["build"], ["GAME.CFG"]]);
        const settings = path.join(folder, "qb64rust.toml");
        assert.deepStrictEqual(result, { settings });
        assert.deepStrictEqual(plainLines(fs.readFileSync(settings, "utf8")), ["[build]", 'output = "build"', 'copy = ["GAME.CFG"]']);
        assert.strictEqual(vscode.window.activeTextEditor?.document.uri.fsPath, settings);
    });

    it("every default: the file `qb64rust init --yes` writes", async () => {
        const uri = writeFile("setup/defaults/game.bas", "PRINT 1\r\n");
        writeFile("setup/twin/game.bas", "PRINT 1\r\n");
        const folder = path.dirname(uri.fsPath);
        await setUp(folder, [["."]]);
        const twin = path.join(workspace, "setup", "twin");
        execFileSync(server, ["init", twin, "--yes"], { windowsHide: true });
        assert.strictEqual(fs.readFileSync(path.join(folder, "qb64rust.toml"), "utf8"), fs.readFileSync(path.join(twin, "qb64rust.toml"), "utf8"));
    });

    it("canceled: a list left without a choice writes nothing", async () => {
        const uri = writeFile("setup/canceled/game.bas", "PRINT 1\r\n");
        writeFile("setup/canceled/other.bas", "PRINT 2\r\n");
        const folder = path.dirname(uri.fsPath);
        assert.strictEqual(await setUp(folder, [["game.bas"], undefined]), undefined);
        assert.ok(!fs.existsSync(path.join(folder, "qb64rust.toml")));
    });

    it("a settings file is there: it opens, and nothing is written", async () => {
        const uri = writeFile("setup/there/game.bas", "PRINT 1\r\n");
        const toml = writeFile("setup/there/qb64rust.toml", "[lints]\nall = true\n");
        assert.deepStrictEqual(await setUp(path.dirname(uri.fsPath)), { message: "set up already" });
        assert.strictEqual(fs.readFileSync(toml.fsPath, "utf8"), "[lints]\nall = true\n");
        assert.strictEqual(vscode.window.activeTextEditor?.document.uri.fsPath, toml.fsPath);
    });

    it("an empty folder: what QB64: New Project writes", async () => {
        const folder = path.join(workspace, "setup", "empty");
        fs.mkdirSync(folder, { recursive: true });
        await setUp(folder);
        assert.strictEqual(fs.readFileSync(path.join(folder, "main.bas"), "latin1"), MAIN_BAS);
        assert.strictEqual(fs.readFileSync(path.join(folder, "qb64rust.toml"), "utf8"), SETTINGS_TOML);
    });

    // Design D7: `qb64rust init --yes` in an empty folder writes what New Project writes: the same `main.bas` and
    // `.vscode/settings.json`, and a settings file with the same settings.
    it("qb64rust init in an empty folder matches New Project", () => {
        const folder = path.join(workspace, "setup", "init-empty");
        fs.mkdirSync(folder, { recursive: true });
        execFileSync(server, ["init", folder, "--yes"], { windowsHide: true });
        assert.strictEqual(fs.readFileSync(path.join(folder, "main.bas"), "latin1"), MAIN_BAS);
        assert.strictEqual(fs.readFileSync(path.join(folder, ".vscode", "settings.json"), "latin1"), WORKSPACE_SETTINGS);
        const sorted = (lines: string[]) => [...lines].sort();
        assert.deepStrictEqual(sorted(plainLines(fs.readFileSync(path.join(folder, "qb64rust.toml"), "utf8"))), sorted(plainLines(SETTINGS_TOML)));
    });
});
