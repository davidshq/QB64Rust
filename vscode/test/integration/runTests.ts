// Starts a VS Code instance with the extension loaded and runs the integration suite in it.
// The workspace is a fresh copy of test-fixtures/workspace, because the tests save and build files.
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import { runTests } from "@vscode/test-electron";

async function main(): Promise<void> {
    // Set when this runs from a terminal inside VS Code; it would start the test instance as plain Node.
    delete process.env.ELECTRON_RUN_AS_NODE;
    const root = path.resolve(__dirname, "../../..");

    // Point the workspace at the compiler: QB64RUST_TEST_QB64PE (CI), else the reference clone if it exists.
    // Compiler tests skip when there is none, but a QB64RUST_TEST_QB64PE that does not exist is an error.
    const fromEnv = process.env.QB64RUST_TEST_QB64PE;
    const compiler = fromEnv ? path.resolve(fromEnv) : path.resolve(root, "..", "..", "QB64pe", process.platform === "win32" ? "qb64pe.exe" : "qb64pe");
    if (fromEnv && !fs.existsSync(compiler)) {
        throw new Error(`QB64RUST_TEST_QB64PE=${fromEnv}: no such file`);
    }
    // The new compiler's binary for the language server tests: QB64RUST_TEST_QB64RUST only (no fallback, so a run
    // without it is the M1 extension as before). Those tests skip when it is unset; one that does not exist is an
    // error.
    const serverEnv = process.env.QB64RUST_TEST_QB64RUST;
    const server = serverEnv ? path.resolve(serverEnv) : "";
    if (server && !fs.existsSync(server)) {
        throw new Error(`QB64RUST_TEST_QB64RUST=${serverEnv}: no such file`);
    }
    const workspace = fs.mkdtempSync(path.join(os.tmpdir(), "qb64rust-it-"));
    fs.cpSync(path.join(root, "test-fixtures", "workspace"), workspace, { recursive: true });

    const settingsDir = path.join(workspace, ".vscode");
    fs.mkdirSync(settingsDir, { recursive: true });
    const settings: Record<string, unknown> = { "qb64rust.noPrompt": true };
    if (fs.existsSync(compiler)) {
        settings["qb64rust.compilerPath"] = compiler;
    }
    // Without a binary named, one on PATH is not used either: the run is the M1 extension.
    settings["qb64rust.path"] = server || path.join(workspace, "no-qb64rust-here");
    fs.writeFileSync(path.join(settingsDir, "settings.json"), JSON.stringify(settings, null, 2));

    // `--min` tests the oldest version package.json accepts (engines.vscode), as rust-analyzer does.
    const engine = JSON.parse(fs.readFileSync(path.join(root, "package.json"), "utf8")).engines.vscode as string;
    const version = process.argv.includes("--min") ? engine.replace(/^[^\d]*/, "") : (process.env.QB64RUST_TEST_VSCODE_VERSION ?? "stable");

    try {
        await runTests({
            version,
            extensionDevelopmentPath: root,
            extensionTestsPath: path.join(__dirname, "suite", "index.js"),
            extensionTestsEnv: {
                QB64RUST_TEST_WORKSPACE: workspace,
                QB64RUST_TEST_COMPILER: fs.existsSync(compiler) ? compiler : "",
                QB64RUST_TEST_SERVER: server,
            },
            launchArgs: [workspace, "--disable-extensions", "--disable-workspace-trust"],
        });
    } finally {
        fs.rmSync(workspace, { recursive: true, force: true });
    }
}

main().catch((err) => {
    console.error(err);
    process.exit(1);
});
