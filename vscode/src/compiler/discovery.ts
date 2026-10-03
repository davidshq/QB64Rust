// Finds the compiler: the `qb64rust.compilerPath` setting, then `qb64pe` on PATH. Pure apart from the injected
// file-existence check, so it is unit-tested without VS Code.
import * as fs from "fs";
import * as path from "path";

export type Discovery =
    | { found: true; path: string; source: "setting" | "PATH" }
    | { found: false; reason: string };

export interface DiscoveryInput {
    /** Value of `qb64rust.compilerPath`, possibly empty. */
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

export function findCompiler(input: DiscoveryInput): Discovery {
    const isFile = input.isFile ?? defaultIsFile;
    const p = input.platform === "win32" ? path.win32 : path.posix;
    const exe = input.platform === "win32" ? "qb64pe.exe" : "qb64pe";

    if (input.setting) {
        let configured = input.setting;
        if (!p.isAbsolute(configured) && input.baseDir) {
            configured = p.resolve(input.baseDir, configured);
        }
        if (isFile(configured)) {
            return { found: true, path: configured, source: "setting" };
        }
        // A folder is accepted too: the QB64pe installation directory.
        const inside = p.join(configured, exe);
        if (isFile(inside)) {
            return { found: true, path: inside, source: "setting" };
        }
        return { found: false, reason: `qb64rust.compilerPath points to "${input.setting}", which does not exist.` };
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
    return { found: false, reason: "qb64pe was not found: set qb64rust.compilerPath or add the QB64pe folder to PATH." };
}
