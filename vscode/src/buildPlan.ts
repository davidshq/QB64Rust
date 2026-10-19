// Which compiler builds a program and what it is given (design D3, D5 and D6 of extension-vsix), shared by Build,
// Build and Run and the build task so they cannot differ (spec editor/build-run, "Tasks").
import * as fs from "fs";
import * as path from "path";
import * as vscode from "vscode";
import * as config from "./config";
import { objectsLock, prepareLibqb } from "./compiler/libqbStore";
import * as qb64pe from "./compiler/qb64pe";
import * as qb64rust from "./compiler/qb64rust";
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
    | { compiler: "qb64pe"; command: string; args: string[] }
    | {
          compiler: "qb64rust";
          command: string;
          args: string[];
          roots: qb64rust.Roots;
          env: NodeJS.ProcessEnv;
          /** No libqb object built yet in the prepared copy: this build makes them (about 40 s more). */
          firstLibqbBuild: boolean;
          /** The QB64pe executable, for the fallback of "Build with QB64pe". */
          qb64pe: string;
      };

/** No build: why, already shown to the user when the plan was interactive. */
export type NoBuild = { compiler: "none"; reason: string };

/** The compiler `qb64rust.buildWith` picks now, given whether the `qb64rust` binary is found. */
export function chosenCompiler(server: LanguageServer, uri?: vscode.Uri): "qb64rust" | "qb64pe" {
    const setting = config.buildWith(uri);
    if (setting !== "auto") {
        return setting;
    }
    return server.binary.found ? "qb64rust" : "qb64pe";
}

/**
 * The build of `file`, or why there is none, told to the user: QB64pe not found (needed in every case, for its
 * toolchain), `qb64rust` chosen but not found, or the copy of libqb could not be prepared. With `interactive`
 * false nothing is shown (a task listing).
 */
export async function planBuild(file: string, status: CompilerStatus, server: LanguageServer, bundle: Bundle, interactive = true): Promise<BuildPlan | NoBuild> {
    const current = status.current;
    if (!current.found) {
        if (interactive) {
            await status.require(); // says so, with a button to the setting
        }
        return { compiler: "none", reason: current.reason };
    }
    const compiler = current.path;
    const uri = vscode.Uri.file(file);
    if (chosenCompiler(server, uri) === "qb64pe") {
        return { compiler: "qb64pe", command: compiler, args: qb64pe.buildArgs(file) };
    }
    const binary = server.binary;
    if (!binary.found) {
        const reason = `qb64rust.buildWith is "qb64rust", but ${binary.reason}`;
        if (interactive) {
            const open = "Open Setting";
            const choice = await vscode.window.showErrorMessage(`QB64: ${reason}`, open);
            if (choice === open) {
                await vscode.commands.executeCommand("workbench.action.openSettings", "qb64rust.path");
            }
        }
        return { compiler: "none", reason };
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
    const qb64peFolder = path.dirname(compiler);
    const roots: qb64rust.Roots = { qb64pe: qb64peFolder, libqb, include: includeRootFor(config.includeRoot(), baseDir, qb64peFolder, process.platform) };
    return {
        compiler: "qb64rust",
        command: binary.path,
        args: qb64rust.buildArgs(file, roots),
        roots,
        env,
        firstLibqbBuild: libqb !== undefined && !hasObjects(libqb),
        qb64pe: compiler,
    };
}

function hasObjects(libqb: string): boolean {
    try {
        return fs.readdirSync(path.join(libqb, "internal", "c")).some((n) => n.endsWith(".o"));
    } catch {
        return false;
    }
}
