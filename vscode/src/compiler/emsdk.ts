// The Emscripten SDK of a build for the browser, and the plan of "QB64: Run in Browser" (OpenSpec change
// wasm-publish, design D5; spec editor/packaging, "Emscripten SDK lookup in the extension", "Emscripten SDK offer"
// and "Emscripten SDK install command"; spec editor/build-run, "Run in browser"). The setting `qb64rust.emsdkPath`,
// else what `qb64rust toolchain path --target wasm` reports; when none is found, the user is asked before anything
// is downloaded. Free of VS Code, as toolchain.ts is: the window and the processes are given.
import { InstallRun, ProcessRun, DOWNLOAD, NOT_NOW, SHOW_OUTPUT } from "./toolchain";
import { Roots, rootArgs } from "./qb64rust";

export type EmsdkSource = "setting" | "option" | "variable" | "installed";

/** Not found: `installable` when the lookup ran and found none, so that an install would give one; else `setting`
 * names what to change. */
export type EmsdkDiscovery = { found: true; folder: string; source: EmsdkSource } | { found: false; reason: string; installable?: boolean; setting?: string };

/** The setting that names an Emscripten SDK folder of one's own. */
export const SETTING = "qb64rust.emsdkPath";
/** The command that offers the SDK on demand. */
export const INSTALL_COMMAND = "qb64rust.installEmsdk";
export const INSTALL_TITLE = "QB64: Install the Emscripten SDK";
/** The command that builds for the browser, serves the page and opens it. */
export const RUN_COMMAND = "qb64rust.runInBrowser";
/** The terminal the page's server runs in. */
export const TERMINAL_NAME = "QB64 Web";

/** The start of `toolchain.rs` `emsdk_missing_message`: the lookup ran and found nothing. */
const MISSING = "no Emscripten SDK found";

/** The question of spec editor/packaging, "Emscripten SDK offer": the source, the download's size and the disk
 * space (`toolchain.rs` `EMSDK_DOWNLOAD_MB` and `EMSDK_DISK_MB`). */
export const OFFER =
    "Building for the browser needs the Emscripten SDK (Emscripten 7.0.0 with its Node and Python, from storage.googleapis.com/webassembly): an 815 MB download, about 2.2 GB on disk. Download it now?";

/** What a declined offer says: the command and the setting. */
export const DECLINED = `QB64: no Emscripten SDK, so nothing was built for the browser. Install it with "${INSTALL_TITLE}", or set ${SETTING} to an emsdk folder of your own.`;

/** A `$CONSOLE:ONLY` program has no page (spec editor/build-run, "Run in browser", scenario "A console program"). */
export const NO_PAGE = "QB64: this program is $CONSOLE:ONLY, so it has no page. Its wasm build runs under Node: node build/<name>.js.";

/** The browser build is qb64rust's only (scenario "Built with QB64pe"). */
export const NEEDS_QB64RUST = "the browser build needs qb64rust, and qb64rust.buildWith is set to qb64pe.";

export interface EmsdkInput {
    /** `qb64rust.emsdkPath`, possibly empty. */
    setting: string;
    /** The `qb64rust` binary, when found. */
    binary?: string;
    /** Runs the binary with the arguments; given by the caller (a child process). */
    run: (binary: string, args: string[]) => Promise<ProcessRun>;
}

/** `qb64rust toolchain path --target wasm`'s output: the folder on the first line, the source's word before the
 * colon on the second; with a non-zero exit code, its message. */
export function parseEmsdkPath(run: ProcessRun): EmsdkDiscovery {
    const lines = run.output.split(/\r?\n/).filter((l) => l.trim() !== "");
    if (run.exitCode === 0 && lines.length >= 2) {
        const word = lines[1].split(":", 1)[0].trim();
        const sources: EmsdkSource[] = ["option", "variable", "installed"];
        if ((sources as string[]).includes(word)) {
            return { found: true, folder: lines[0].trim(), source: word as EmsdkSource };
        }
    }
    const message = lines.join(" ").replace(/^qb64rust: /, "");
    if (run.exitCode === 1 && message.startsWith(MISSING)) {
        return { found: false, reason: message, installable: true };
    }
    return { found: false, reason: message || `qb64rust toolchain path --target wasm failed (exit code ${run.exitCode})` };
}

/**
 * The SDK a build for the browser uses: `qb64rust.emsdkPath`, else what `qb64rust toolchain path --target wasm`
 * reports. The setting's folder is checked by the compiler too (`--emsdk-root`), which knows the pinned release; a
 * folder it refuses is not found, naming the setting, and no download is offered, since the setting would still win.
 */
export async function findEmsdk(input: EmsdkInput): Promise<EmsdkDiscovery> {
    if (!input.binary) {
        return { found: false, reason: "the qb64rust binary was not found, so no Emscripten SDK was looked for.", setting: "qb64rust.path" };
    }
    if (input.setting) {
        const found = parseEmsdkPath(await input.run(input.binary, ["toolchain", "path", "--target", "wasm", "--emsdk-root", input.setting]));
        return found.found ? { found: true, folder: found.folder, source: "setting" } : { found: false, reason: `${SETTING}: ${found.reason}`, setting: SETTING };
    }
    return parseEmsdkPath(await input.run(input.binary, ["toolchain", "path", "--target", "wasm"]));
}

/** What the offer needs from the window and the processes. */
export interface EmsdkUi {
    /** The modal question; resolves to the choice, undefined when it was closed. */
    ask(message: string, ...choices: string[]): Thenable<string | undefined>;
    /** Runs `qb64rust toolchain install --target wasm` with a cancellable progress notification. */
    install(): Promise<InstallRun>;
    /** A message with buttons; resolves to the one chosen. */
    error(message: string, ...items: string[]): Thenable<string | undefined>;
    info(message: string): Thenable<unknown>;
    showOutput(): void;
    /** Looks for the SDK again once it is installed. */
    find(): Promise<EmsdkDiscovery>;
    /** The platform (default: Windows); on Linux nothing is downloaded. */
    platform?: string;
}

/** The last line of an install's output that says what went wrong. */
function failure(output: string): string {
    const lines = output.split(/\r?\n/).filter((l) => l.trim() !== "");
    return (lines.at(-1) ?? "no output").replace(/^qb64rust: /, "");
}

function showFailure(ui: EmsdkUi, message: string): void {
    void ui.error(message, SHOW_OUTPUT).then((c) => c === SHOW_OUTPUT && ui.showOutput());
}

/**
 * Spec editor/packaging, "Emscripten SDK offer": asks, and on "Download" installs and looks again. Resolves to the
 * SDK found after the install, or undefined (declined, cancelled, failed, or Linux, where the compiler's message is
 * shown and nothing is downloaded); what happened has been shown.
 */
export async function offerEmsdk(ui: EmsdkUi, missing?: EmsdkDiscovery): Promise<EmsdkDiscovery | undefined> {
    if ((ui.platform ?? "win32") !== "win32") {
        showFailure(ui, `QB64: ${missing && !missing.found ? missing.reason : MISSING}`);
        return undefined;
    }
    const choice = await ui.ask(OFFER, DOWNLOAD, NOT_NOW);
    if (choice !== DOWNLOAD) {
        void ui.info(DECLINED);
        return undefined;
    }
    const run = await ui.install();
    if (run.cancelled) {
        void ui.info("QB64: the Emscripten SDK download was cancelled; nothing is installed. What was downloaded is removed by the next install.");
        return undefined;
    }
    if (run.exitCode !== 0) {
        showFailure(ui, `QB64: installing the Emscripten SDK failed: ${failure(run.output)}`);
        return undefined;
    }
    const found = await ui.find();
    if (!found.found) {
        showFailure(ui, `QB64: the Emscripten SDK was installed but is not found: ${found.reason}`);
        return undefined;
    }
    return found;
}

/**
 * Spec editor/packaging, "Emscripten SDK install command": in an untrusted workspace a message and nothing else;
 * with an SDK found, a message naming its folder; with a setting that names no SDK, the reason; otherwise the offer.
 */
export async function installCommand(trusted: boolean, ui: EmsdkUi): Promise<EmsdkDiscovery | undefined> {
    if (!trusted) {
        void ui.info("QB64: installing the Emscripten SDK requires a trusted workspace. Nothing was run.");
        return undefined;
    }
    const now = await ui.find();
    if (now.found) {
        void ui.info(`QB64: the Emscripten SDK is found already: ${now.folder}`);
        return now;
    }
    if (!now.installable) {
        void ui.error(`QB64: ${now.reason}`);
        return undefined;
    }
    return offerEmsdk(ui, now);
}

/** `--print-paths <file> --target wasm`: asks the compiler where a window program's page goes (spec compiler/cli,
 * "The paths of a program"). */
export function pagePathArgs(file: string): string[] {
    return ["--print-paths", file, "--target", "wasm"];
}

/** The `page` of the one line of JSON that `--print-paths --target wasm` prints, or undefined when the line has
 * none: a compiler from before the web folder, or a settings file's error. */
export function parsePagePath(output: string): string | undefined {
    try {
        const json: unknown = JSON.parse(output.trim());
        if (typeof json === "object" && json !== null) {
            const { page } = json as Record<string, unknown>;
            return typeof page === "string" && page ? page : undefined;
        }
    } catch {
        // Not JSON.
    }
    return undefined;
}

/** `-x -w --target wasm <file> --emsdk-root <sdk>` and the roots a desktop build passes. No `-o`: the compiler
 * writes the web folder only without it. */
export function wasmBuildArgs(file: string, roots: Roots, emsdk: string): string[] {
    return ["-x", "-w", "--target", "wasm", file, "--emsdk-root", emsdk, ...rootArgs(roots)];
}

/** Whether a build that started at `started` wrote the page, whose modification time is `modified` (undefined: no
 * such file). A `$CONSOLE:ONLY` program builds without one, and an older page may still be there. Both in
 * milliseconds; a file system may round a time down, by up to two seconds on FAT. */
export function pageWasBuilt(modified: number | undefined, started: number): boolean {
    return modified !== undefined && modified >= started - 2000;
}

/** `serve <folder>`, with the port of the run before so that the address, and with it the page's saved games,
 * stays the same (scenario "A second run"). */
export function serveArgs(folder: string, port?: number): string[] {
    return ["serve", folder, ...(port === undefined ? [] : ["--port", String(port)])];
}

/** The address `qb64rust serve` prints on its first line, and its port; undefined for any other line. */
export function parseAddress(line: string): { url: string; port: number } | undefined {
    const m = /^(http:\/\/127\.0\.0\.1:(\d+)\/)\s*$/.exec(line);
    return m ? { url: m[1], port: Number(m[2]) } : undefined;
}
