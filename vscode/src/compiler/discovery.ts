// Finds a program: its setting (`qb64rust.compilerPath` for `qb64pe`, `qb64rust.path` for `qb64rust`), then the
// program on PATH. Pure apart from the injected file-existence check, so it is unit-tested without VS Code.
import * as fs from "fs";
import * as path from "path";

export type Discovery =
    | { found: true; path: string; source: "setting" | "PATH" }
    | { found: false; reason: string };

/** What to look for: the executable's name (without `.exe`), its setting, and what to say when it is not found. */
export interface Program {
    name: string;
    setting: string;
    hint: string;
}

export const QB64PE: Program = { name: "qb64pe", setting: "qb64rust.compilerPath", hint: "add the QB64pe folder to PATH" };
export const QB64RUST: Program = { name: "qb64rust", setting: "qb64rust.path", hint: "add the folder of qb64rust to PATH" };

export interface DiscoveryInput {
    /** Value of the program's setting, possibly empty. */
    setting: string;
    /** Folder a relative setting is resolved against (the first workspace folder), if any. */
    baseDir?: string;
    /** The PATH environment variable. */
    envPath: string | undefined;
    platform: NodeJS.Platform;
    isFile?: (p: string) => boolean;
}

function defaultIsFile(p: string): boolean {
    try {
        return fs.statSync(p).isFile();
    } catch {
        return false;
    }
}

export function findCompiler(input: DiscoveryInput, program: Program = QB64PE): Discovery {
    const isFile = input.isFile ?? defaultIsFile;
    const p = input.platform === "win32" ? path.win32 : path.posix;
    const exe = input.platform === "win32" ? `${program.name}.exe` : program.name;

    if (input.setting) {
        let configured = input.setting;
        if (!p.isAbsolute(configured) && input.baseDir) {
            configured = p.resolve(input.baseDir, configured);
        }
        if (isFile(configured)) {
            return { found: true, path: configured, source: "setting" };
        }
        // A folder is accepted too: the installation directory.
        const inside = p.join(configured, exe);
        if (isFile(inside)) {
            return { found: true, path: inside, source: "setting" };
        }
        return { found: false, reason: `${program.setting} points to "${input.setting}", which does not exist.` };
    }

    const sep = input.platform === "win32" ? ";" : ":";
    for (const dir of (input.envPath ?? "").split(sep)) {
        const d = dir.trim().replace(/^"(.*)"$/, "$1");
        if (!d) {
            continue;
        }
        const candidate = p.join(d, exe);
        if (isFile(candidate)) {
            return { found: true, path: candidate, source: "PATH" };
        }
    }
    return { found: false, reason: `${program.name} was not found: set ${program.setting} or ${program.hint}.` };
}
