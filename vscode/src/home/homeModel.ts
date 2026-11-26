// The rules of the page "QB64 Home" (OpenSpec change editor-side-panel, design D11; spec editor/home): when it opens
// by itself, the list of recent programs, and the rows of the setup's state. No VS Code import, so it is
// unit-tested without it.
import * as path from "path";

/** How many recent programs are kept. */
export const MAX_RECENT = 8;

export interface WindowInputs {
    /** The setting `qb64rust.showHomeOnStartup`, which the page's checkbox changes. */
    enabled: boolean;
    /** The number of workspace folders. */
    folders: number;
    /** Whether a workspace folder holds a `.bas`, `.bi` or `.bm` file. */
    hasQb64Files: boolean;
    /** The number of files open in editors (text, diff, notebook and custom editors; not VS Code's Welcome page). */
    openFiles: number;
}

/**
 * Whether the page opens by itself when a window starts: the setting is on, no file is open, and the window is
 * one for QB64: it has no folder, or a folder with QB64 files. A window of another project never shows it.
 */
export function shouldOpenHome(i: WindowInputs): boolean {
    return i.enabled && i.openFiles === 0 && (i.folders === 0 || i.hasQb64Files);
}

function sameFile(a: string, b: string, platform: NodeJS.Platform): boolean {
    return platform === "win32" ? a.toLowerCase() === b.toLowerCase() : a === b;
}

/** `list` with `file` first, once, and at most `MAX_RECENT` long. */
export function addRecent(list: string[], file: string, platform: NodeJS.Platform = process.platform): string[] {
    return [file, ...list.filter((f) => !sameFile(f, file, platform))].slice(0, MAX_RECENT);
}

/** Whether `file` is in `folder` or below it. */
export function isInside(file: string, folder: string): boolean {
    const rel = path.relative(folder, file);
    return rel !== "" && !rel.startsWith("..") && !path.isAbsolute(rel);
}

export interface RecentRow {
    path: string;
    /** The file's name: `bounce.bas`. */
    name: string;
    /** The name of the folder it is in: `games`. */
    folder: string;
}

/** The recent programs that are still there, newest first. */
export function recentRows(list: string[], exists: (file: string) => boolean): RecentRow[] {
    return list.filter(exists).map((file) => ({ path: file, name: path.basename(file), folder: path.basename(path.dirname(file)) }));
}

/** A command the page sends (`data-command`). */
export type HomeCommand =
    | "newProgram"
    | "openProgram"
    | "examples"
    | "searchHelp"
    | "getStarted"
    | "recent"
    | "page"
    | "installToolchain"
    | "installEmsdk"
    | "toggle";

export interface SetupRow {
    ok: boolean;
    label: string;
    /** What is there, or what is missing. */
    note: string;
    /** A link that installs what is missing. */
    action?: { command: HomeCommand; text: string };
}

export interface SetupInputs {
    /** Whether the `qb64rust` binary was found. */
    compiler: boolean;
    toolchain: { found: boolean; installable?: boolean };
    emsdk: { found: boolean; installable?: boolean };
    platform: NodeJS.Platform;
}

/** The three rows of "Setup": the compiler, the C++ compiler a build needs, and the optional SDK for the web. */
export function setupRows(i: SetupInputs): SetupRow[] {
    const linux = i.platform === "linux";
    const toolchain: SetupRow = i.toolchain.found
        ? { ok: true, label: "The C++ compiler", note: linux ? "your system's g++" : "installed" }
        : {
              ok: false,
              label: "The C++ compiler",
              note: linux ? "g++ is missing; the first run names the packages to install" : "needed to run a program; the first run offers it",
              action: i.toolchain.installable && !linux ? { command: "installToolchain", text: "Install (190 MB)" } : undefined,
          };
    const emsdk: SetupRow = i.emsdk.found
        ? { ok: true, label: "Programs for the web", note: "installed" }
        : {
              ok: false,
              label: "Programs for the web",
              note: "optional",
              action: i.emsdk.installable ? { command: "installEmsdk", text: "Install (815 MB)" } : undefined,
          };
    return [
        { ok: i.compiler, label: "The QB64 compiler", note: i.compiler ? "found" : "not found: checking and running are off" },
        toolchain,
        emsdk,
    ];
}

export interface HomeState {
    recent: RecentRow[];
    setup: SetupRow[];
    /** The setting behind the checkbox. */
    showOnStartup: boolean;
}
