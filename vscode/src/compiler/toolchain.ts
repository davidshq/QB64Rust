// The C++ toolchain of a build and a debug session with qb64rust (change m6-toolchain, design D6; spec
// editor/packaging, "Toolchain lookup in the extension" and "Toolchain offer"). The setting `qb64rust.toolchainPath`,
// else what `qb64rust toolchain path` reports; when none is found, the user is asked before anything is downloaded,
// and only "Download" runs `qb64rust toolchain install`. Free of VS Code: the window and the processes are given,
// so that the order and the answers are unit-tested.
import * as fs from "fs";
import * as path from "path";

export type ToolchainSource = "setting" | "option" | "variable" | "qb64pe" | "installed" | "clone" | "system";

/** The name of the bundled binary on a platform (design D8 of m7-linux): `qb64rust.exe` on Windows, `qb64rust` on
 * Linux. */
export function binaryName(platform: string = process.platform): string {
    return platform === "win32" ? "qb64rust.exe" : "qb64rust";
}

/** Not found: `installable` when `qb64rust toolchain path` ran and found nothing, so that an install would give one;
 * else (a setting or a variable naming a folder that is not a toolchain, no binary) `setting` names what to change. */
export type ToolchainDiscovery =
    | { found: true; folder: string; source: ToolchainSource }
    | { found: false; reason: string; installable?: boolean; setting?: string };

/** The `--toolchain-root` a build and the adapter are given for a toolchain found: its folder, or none for the
 * system's (Linux), whose folder `qb64rust toolchain path` gives as the folder of `g++` (`/usr/bin`), which holds no
 * `bin/g++`; the compiler then finds it on `PATH` again (m7-linux, task 6.3). */
export function toolchainRoot(found: { folder: string; source: ToolchainSource }): string | undefined {
    return found.source === "system" ? undefined : found.folder;
}

/** The start of `toolchain.rs` `missing_message`: the lookup ran and found nothing. */
const MISSING = "no C++ toolchain found";

/** A finished run of a process: its exit code (null when it did not start or was killed) and what it printed. */
export interface ProcessRun {
    exitCode: number | null;
    output: string;
}

export interface ToolchainInput {
    /** `qb64rust.toolchainPath`, possibly empty. */
    setting: string;
    /** The `qb64rust` binary, when found. */
    binary?: string;
    /** The folder of the QB64pe found by `qb64rust.compilerPath` or PATH, if any. */
    qb64pe?: string;
    /** Runs the binary with the arguments; given by the caller (a child process). */
    run: (binary: string, args: string[]) => Promise<ProcessRun>;
    isToolchain?: (folder: string) => boolean;
    /** The platform (default: this process's); on Linux a toolchain folder holds `bin/g++` and `bin/make`. */
    platform?: string;
}

/** The setting that names a toolchain folder of one's own. */
export const SETTING = "qb64rust.toolchainPath";
/** The command that offers the toolchain on demand. */
export const INSTALL_COMMAND = "qb64rust.installToolchain";
export const INSTALL_TITLE = "QB64: Install the C++ Toolchain";

export const DOWNLOAD = "Download";
export const NOT_NOW = "Not Now";

/** The question of spec editor/packaging, "Toolchain offer": the source, the download's size and the disk space. */
export const OFFER =
    "Building and debugging need a C++ toolchain (llvm-mingw 20260922 with LLVM 23.1.2, from github.com/mstorsjo/llvm-mingw): a 190 MB download, about 460 MB on disk. Download it now?";

/** What a declined offer says: the command and the setting. */
export const DECLINED = `QB64: no C++ toolchain, so nothing was started. Install it with "${INSTALL_TITLE}", or set ${SETTING} to a toolchain folder of your own.`;

/** The programs a toolchain folder holds on a platform (as `toolchain.rs` `is_toolchain`). */
function toolchainPrograms(platform: string): [string, string] {
    return platform === "win32" ? ["c++.exe", "mingw32-make.exe"] : ["g++", "make"];
}

/** Whether `folder` holds the programs a build runs (as `toolchain.rs` `is_toolchain`). */
export function isToolchainFolder(folder: string, platform: string = process.platform): boolean {
    const isFile = (p: string) => {
        try {
            return fs.statSync(p).isFile();
        } catch {
            return false;
        }
    };
    return toolchainPrograms(platform).every((name) => isFile(path.join(folder, "bin", name)));
}

/** `qb64rust toolchain path`'s output: the folder on the first line, the source's word before the colon on the
 * second; with a non-zero exit code, its message. */
export function parseToolchainPath(run: ProcessRun): ToolchainDiscovery {
    const lines = run.output.split(/\r?\n/).filter((l) => l.trim() !== "");
    if (run.exitCode === 0 && lines.length >= 2) {
        const word = lines[1].split(":", 1)[0].trim();
        const sources: ToolchainSource[] = ["option", "variable", "qb64pe", "installed", "clone", "system"];
        if ((sources as string[]).includes(word)) {
            return { found: true, folder: lines[0].trim(), source: word as ToolchainSource };
        }
    }
    const message = lines.join(" ").replace(/^qb64rust: /, "");
    if (run.exitCode === 1 && message.startsWith(MISSING)) {
        return { found: false, reason: message, installable: true };
    }
    return { found: false, reason: message || `qb64rust toolchain path failed (exit code ${run.exitCode})` };
}

/** The toolchain a build and a debug session use (design D6): `qb64rust.toolchainPath`, else `qb64rust toolchain
 * path`, given the QB64pe found, so that a QB64pe user's toolchain is QB64pe's. */
export async function findToolchain(input: ToolchainInput): Promise<ToolchainDiscovery> {
    const platform = input.platform ?? process.platform;
    const isToolchain = input.isToolchain ?? ((folder: string) => isToolchainFolder(folder, platform));
    const [cxx, make] = toolchainPrograms(platform).map((name) => path.join("bin", name));
    if (input.setting) {
        return isToolchain(input.setting)
            ? { found: true, folder: input.setting, source: "setting" }
            : { found: false, reason: `${SETTING} points to "${input.setting}", which has no ${cxx} or ${make}.`, setting: SETTING };
    }
    if (!input.binary) {
        return { found: false, reason: "the qb64rust binary was not found, so no toolchain was looked for.", setting: "qb64rust.path" };
    }
    const args = ["toolchain", "path", ...(input.qb64pe ? ["--qb64pe-root", input.qb64pe] : [])];
    return parseToolchainPath(await input.run(input.binary, args));
}

/** The percentage of a `downloading <n>%` line of `qb64rust toolchain install`, else undefined. */
export function downloadPercent(line: string): number | undefined {
    const m = /^downloading (\d+)%$/.exec(line.trim());
    return m ? Number(m[1]) : undefined;
}

/** A run of `qb64rust toolchain install` under the progress notification. */
export interface InstallRun extends ProcessRun {
    cancelled: boolean;
}

/** What the offer needs from the window and the processes. */
export interface OfferUi {
    /** The modal question; resolves to the choice, undefined when it was closed. */
    ask(message: string, ...choices: string[]): Thenable<string | undefined>;
    /** Runs `qb64rust toolchain install` with a cancellable progress notification, its lines to the "QB64" output. */
    install(): Promise<InstallRun>;
    /** A message with buttons; resolves to the one chosen. */
    error(message: string, ...items: string[]): Thenable<string | undefined>;
    info(message: string): Thenable<unknown>;
    showOutput(): void;
    /** Looks for the toolchain again once it is installed. */
    find(): Promise<ToolchainDiscovery>;
    /** The platform (default: Windows, the platform the offer was written for); on Linux nothing is downloaded. */
    platform?: string;
    /** Puts text on the clipboard. */
    copy?(text: string): Thenable<void>;
}

export const SHOW_OUTPUT = "Show Output";
export const COPY_INSTALL = "Copy Install Command";

/** The install line a Linux message names (`toolchain.rs` `packages_message`): the text in backquotes that starts
 * with `sudo `. */
export function installLine(message: string): string | undefined {
    const m = /`(sudo [^`]+)`/.exec(message);
    return m ? m[1] : undefined;
}

/** On Linux (spec editor/packaging, "Linux without g++"; design D8 of m7-linux): the message, which names the
 * packages, with a button that copies the install line and one to the "QB64" output. Nothing is downloaded. */
function showPackages(ui: OfferUi, message: string): void {
    const line = installLine(message);
    const items = line ? [COPY_INSTALL, SHOW_OUTPUT] : [SHOW_OUTPUT];
    void ui.error(`QB64: ${message}`, ...items).then((c) => {
        if (c === COPY_INSTALL && line) {
            void ui.copy?.(line);
        } else if (c === SHOW_OUTPUT) {
            ui.showOutput();
        }
    });
}

/** The last line of an install's output that says what went wrong. */
function failure(output: string): string {
    const lines = output.split(/\r?\n/).filter((l) => l.trim() !== "");
    return (lines.at(-1) ?? "no output").replace(/^qb64rust: /, "");
}

/**
 * Spec editor/packaging, "Toolchain offer": asks, and on "Download" installs and looks again. Resolves to the
 * toolchain found after the install, or undefined (declined, cancelled, failed), when the build or session is not
 * started; what happened has been shown.
 */
export async function offerToolchain(ui: OfferUi, missing?: ToolchainDiscovery): Promise<ToolchainDiscovery | undefined> {
    if ((ui.platform ?? "win32") !== "win32") {
        showPackages(ui, missing && !missing.found ? missing.reason : "no C++ toolchain found");
        return undefined;
    }
    const choice = await ui.ask(OFFER, DOWNLOAD, NOT_NOW);
    if (choice !== DOWNLOAD) {
        void ui.info(DECLINED);
        return undefined;
    }
    const run = await ui.install();
    if (run.cancelled) {
        void ui.info("QB64: the toolchain download was cancelled; nothing is installed. What was downloaded is removed by the next install.");
        return undefined;
    }
    if (run.exitCode !== 0) {
        void ui.error(`QB64: installing the C++ toolchain failed: ${failure(run.output)}`, SHOW_OUTPUT).then((c) => c === SHOW_OUTPUT && ui.showOutput());
        return undefined;
    }
    const found = await ui.find();
    if (!found.found) {
        void ui.error(`QB64: the C++ toolchain was installed but is not found: ${found.reason}`, SHOW_OUTPUT).then((c) => c === SHOW_OUTPUT && ui.showOutput());
        return undefined;
    }
    return found;
}

/**
 * Spec editor/packaging, "Install command": in an untrusted workspace a message and nothing else; with the pinned
 * toolchain installed, a message naming its folder; otherwise the offer. `installed` is the lookup without the
 * setting or a QB64pe folder, which finds the installed toolchain first.
 */
export async function installCommand(trusted: boolean, installed: () => Promise<ToolchainDiscovery>, ui: OfferUi): Promise<ToolchainDiscovery | undefined> {
    if (!trusted) {
        void ui.info("QB64: installing the C++ toolchain requires a trusted workspace. Nothing was run.");
        return undefined;
    }
    if ((ui.platform ?? "win32") !== "win32") {
        // `qb64rust toolchain install` downloads nothing on Linux: it says the system toolchain is used, or names
        // the packages to install (spec editor/packaging, "The command on Linux").
        const run = await ui.install();
        if (run.exitCode === 0) {
            void ui.info(`QB64: ${failure(run.output)}`);
            return ui.find();
        }
        showPackages(ui, failure(run.output));
        return undefined;
    }
    const now = await installed();
    if (now.found && now.source === "installed") {
        void ui.info(`QB64: the C++ toolchain is installed already: ${now.folder}`);
        return now;
    }
    return offerToolchain(ui);
}
