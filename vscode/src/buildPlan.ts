// Which compiler builds a program and what it is given (design D3, D5 and D6 of extension-vsix), shared by Build,
// Build and Run and the build task so they cannot differ (spec editor/build-run, "Tasks").
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import * as config from "./config";
import { objectsLock, prepareLibqb } from "./compiler/libqbStore";
import * as qb64pe from "./compiler/qb64pe";
import * as qb64rust from "./compiler/qb64rust";
import { toolchainRoot } from "./compiler/toolchain";
import { debugBuildRefusal } from "./debugConfig";
import { LanguageServer } from "./languageServer";
import { includeRootFor } from "./server/protocol";
import { CompilerStatus } from "./statusBar";

/** What the extension carries for a build with qb64rust, and where it prepares it. */
export interface Bundle {
    /** `bundle\libqb` in the extension's folder; may not exist (a development host). */
    libqb: string;
    /** The extension's global storage folder. */
    storage: string;
    /** The extension's version with the hash of the bundle's libqb (`copyVersion`), which names the prepared copy. */
    version: string;
}

export type BuildPlan =
    | { compiler: "qb64pe"; command: string; args: string[]; flags: string[] }
    | {
          compiler: "qb64rust";
          command: string;
          args: string[];
          /** The `-f:` settings of `qb64rust.build.*` (`config.buildFlags`); none for `qb64rust test`. */
          flags: string[];
          roots: qb64rust.Roots;
          env: NodeJS.ProcessEnv;
          /** No libqb object built yet in the prepared copy: this build makes them (about 40 s more). */
          firstLibqbBuild: boolean;
          /** The QB64pe executable, when found, for the fallback of "Build with QB64pe". */
          qb64pe?: string;
          /** A debug build (`--debug`, change m5-debug-launch): no fallback to QB64pe. */
          debug: boolean;
          /** Where the program goes and the folder it runs in (change build-output-folder, design D5). */
          paths: qb64rust.ProgramPaths;
      };

/** No build: why, already shown to the user when the plan was interactive. `toolchain`: a build by qb64rust found no
 * C++ toolchain, which an interactive plan offers (a task run plans again so, design D6 of m6-toolchain). */
export type NoBuild = { compiler: "none"; reason: string; toolchain?: boolean };

/** The compiler `qb64rust.buildWith` picks now, given whether the `qb64rust` binary is found. */
export function chosenCompiler(server: LanguageServer, uri?: vscode.Uri): "qb64rust" | "qb64pe" {
    const setting = config.buildWith(uri);
    if (setting !== "auto") {
        return setting;
    }
    return server.binary.found ? "qb64rust" : "qb64pe";
}

/**
 * The build of `file`, or why there is none, told to the user. The compiler chosen comes first (change m6-toolchain,
 * design D6): QB64pe is needed only for a build by QB64pe; a build by `qb64rust` needs its binary and the C++
 * toolchain, which is offered when there is none; the copy of libqb may fail to be prepared. With `interactive`
 * false nothing is shown or offered (a task listing). A `debug` build is only ever by `qb64rust` (design D4 of
 * m5-debug-launch): QB64pe chosen, or the binary not found, is a refusal that names the setting to change.
 */
export async function planBuild(file: string, status: CompilerStatus, server: LanguageServer, bundle: Bundle, interactive = true, debug = false): Promise<BuildPlan | NoBuild> {
    const current = status.current;
    const uri = vscode.Uri.file(file);
    const binary = server.binary;
    const refusal = debug ? debugBuildRefusal(config.buildWith(uri), binary) : undefined;
    if (refusal) {
        if (interactive) {
            await refuse(refusal.message, refusal.setting);
        }
        return { compiler: "none", reason: refusal.message };
    }
    if (!debug && chosenCompiler(server, uri) === "qb64pe") {
        if (!current.found) {
            if (interactive) {
                await status.require(); // says so, with a button to the setting
            }
            return { compiler: "none", reason: current.reason };
        }
        const flags = config.buildFlags(uri);
        return { compiler: "qb64pe", command: current.path, args: qb64pe.buildArgs(file, flags), flags };
    }
    const plan = await planQb64rust(file, status, server, bundle, interactive, debug);
    if (plan.compiler === "none") {
        return plan;
    }
    // Asked only for a build: `qb64rust test` builds into folders of its own (change build-output-folder, design D5).
    // `qb64rust.build.outputFolder` set to `source` builds next to the source with `-o`, as QB64pe does.
    const paths = config.outputFolder(uri) === "source" ? qb64rust.fallbackPaths(file) : await qb64rust.programPaths(plan.command, file);
    const flags = config.buildFlags(uri);
    return { ...plan, paths, flags, args: qb64rust.buildArgs(file, plan.roots, debug, paths, flags) };
}

/**
 * The build of `file` by `qb64rust`, whatever `qb64rust.buildWith` says (a build that chose it, and `qb64rust test`,
 * change basic-unit-tests): its binary, the C++ toolchain (offered when there is none and `interactive`), the
 * prepared copy of libqb and the roots; or why there is none.
 */
export async function planQb64rust(file: string, status: CompilerStatus, server: LanguageServer, bundle: Bundle, interactive = true, debug = false): Promise<Extract<BuildPlan, { compiler: "qb64rust" }> | NoBuild> {
    const current = status.current;
    const binary = server.binary;
    if (!binary.found) {
        // Also the Test Explorer's, whatever `qb64rust.buildWith` says.
        const reason = `the qb64rust binary is needed, but ${binary.reason}`;
        if (interactive) {
            await refuse(reason, "qb64rust.path");
        }
        return { compiler: "none", reason };
    }
    const tools = status.toolchain ? await status.toolchain.ensure(interactive) : undefined;
    if (!tools?.found) {
        // Declined, cancelled or failed: shown by the offer, or a setting naming no toolchain: shown with its button.
        // Not interactive: nothing shown yet.
        return { compiler: "none", reason: tools?.reason ?? "the C++ toolchain was not looked for", toolchain: true };
    }
    const env: NodeJS.ProcessEnv = { ...process.env };
    let libqb: string | undefined;
    if (fs.existsSync(path.join(bundle.libqb, "internal", "c", "qbx.cpp"))) {
        try {
            libqb = await prepareLibqb(bundle.libqb, bundle.storage, bundle.version);
        } catch (e) {
            const reason = `could not prepare the copy of libqb: ${e instanceof Error ? e.message : String(e)}`;
            if (interactive) {
                void vscode.window.showErrorMessage(`QB64: ${reason}`);
            }
            return { compiler: "none", reason };
        }
        env.QB64RUST_OBJECTS_LOCK = objectsLock(bundle.storage, bundle.version);
    }
    const folder = vscode.workspace.workspaceFolders?.[0]?.uri;
    const baseDir = folder?.scheme === "file" ? folder.fsPath : undefined;
    const qb64peFolder = current.found ? path.dirname(current.path) : undefined;
    const roots: qb64rust.Roots = {
        toolchain: toolchainRoot(tools),
        qb64pe: qb64peFolder,
        libqb,
        include: includeRootFor(config.includeRoot(), baseDir, qb64peFolder, process.platform),
    };
    // Next to the source until `planBuild` asks the compiler; `qb64rust test` needs no paths.
    const paths = qb64rust.fallbackPaths(file);
    return {
        compiler: "qb64rust",
        command: binary.path,
        args: qb64rust.buildArgs(file, roots, debug, paths),
        flags: [],
        roots,
        env,
        firstLibqbBuild: libqb !== undefined && !hasObjects(libqb),
        qb64pe: current.found ? current.path : undefined,
        debug,
        paths,
    };
}

/**
 * Where the program of `file` is and the folder it runs in, for Run and the run task, which build nothing (change
 * build-output-folder, design D5 and D6). When qb64rust is the compiler `qb64rust.buildWith` picks and its binary is
 * found: the newer of the program `qb64rust --print-paths` names and the one next to the source, which "Build with
 * QB64pe" and an older compiler write. Else next to the source, where QB64pe builds it, and where a build with
 * `qb64rust.build.outputFolder` set to `source` puts it.
 */
export async function runPaths(file: string, server: LanguageServer): Promise<qb64rust.ProgramPaths> {
    const binary = server.binary;
    const beside = qb64rust.fallbackPaths(file);
    const uri = vscode.Uri.file(file);
    if (chosenCompiler(server, uri) === "qb64rust" && binary.found && config.outputFolder(uri) === "project") {
        const compiler = await qb64rust.programPaths(binary.path, file);
        return qb64rust.newerProgram(compiler, beside, modifiedTime);
    }
    return beside;
}

/** The modification time of `file` in milliseconds, or undefined when there is no such file. */
function modifiedTime(file: string): number | undefined {
    try {
        return fs.statSync(file).mtimeMs;
    } catch {
        return undefined;
    }
}

/** Shows why there is no build or session, with a button that opens `setting` when there is one. */
export async function refuse(reason: string, setting?: string): Promise<void> {
    const open = "Open Setting";
    const choice = await (setting ? vscode.window.showErrorMessage(`QB64: ${reason}`, open) : vscode.window.showErrorMessage(`QB64: ${reason}`));
    if (setting && choice === open) {
        await vscode.commands.executeCommand("workbench.action.openSettings", setting);
    }
}

function hasObjects(libqb: string): boolean {
    try {
        return fs.readdirSync(path.join(libqb, "internal", "c")).some((n) => n.endsWith(".o"));
    } catch {
        return false;
    }
}
