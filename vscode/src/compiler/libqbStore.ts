// The copy of libqb a build with qb64rust is given (design D3 of extension-vsix). The extension carries it in
// `bundle\libqb` but never writes into its own folder, so before the first build it is copied into the extension's
// global storage, one folder per extension version, and the compiler builds libqb's objects there. No `vscode`
// import, so it is unit-tested on a temporary folder.
import * as crypto from "crypto";
import * as fs from "fs";
import * as path from "path";

/** Written last into a finished copy: a folder without it is an interrupted copy and is made again. */
const COMPLETE = ".qb64rust-complete";
const PREFIX = "libqb-";

/**
 * What names the copy: the extension's version and the hash of libqb's files that staging wrote beside the bundle's
 * copy (`bundle\libqb.id`), so that a package of the same version with other files of libqb gets a copy of its
 * own. Without the file (a development host), the version alone.
 */
export function copyVersion(version: string, source: string): string {
    try {
        const id = fs.readFileSync(path.join(path.dirname(source), "libqb.id"), "utf-8").trim();
        return /^[0-9a-f]+$/.test(id) ? `${version}-${id}` : version;
    } catch {
        return version;
    }
}

/** The folder of this version's copy. */
export function libqbFolder(storage: string, version: string): string {
    return path.join(storage, PREFIX + version);
}

/** The lock file of this version's libqb objects, for `QB64RUST_OBJECTS_LOCK` (two windows building at once). */
export function objectsLock(storage: string, version: string): string {
    return path.join(storage, `${PREFIX}${version}.lock`);
}

function isComplete(folder: string): boolean {
    return fs.existsSync(path.join(folder, COMPLETE));
}

/**
 * Returns this version's copy of `source` in `storage`, making it first if it is missing or incomplete. The copy
 * is made in a temporary sibling and renamed into place, so a copy cut short is never used, and two windows that
 * copy at once each finish their own (the second rename finds the first's copy and drops its own).
 */
export async function prepareLibqb(source: string, storage: string, version: string): Promise<string> {
    const target = libqbFolder(storage, version);
    if (isComplete(target)) {
        return target;
    }
    if (!fs.existsSync(path.join(source, "internal", "c", "qbx.cpp"))) {
        throw new Error(`the extension's copy of libqb is missing (${source})`);
    }
    await fs.promises.mkdir(storage, { recursive: true });
    const temp = `${target}.tmp-${process.pid}-${crypto.randomBytes(4).toString("hex")}`;
    try {
        // New timestamps: removeOtherVersions takes a temporary folder's age from its time.
        await fs.promises.cp(source, temp, { recursive: true, preserveTimestamps: false });
        await fs.promises.writeFile(path.join(temp, COMPLETE), "");
        if (fs.existsSync(target) && !isComplete(target)) {
            await fs.promises.rm(target, { recursive: true, force: true });
        }
        try {
            await renameRetrying(temp, target);
        } catch (e) {
            if (!isComplete(target)) {
                throw e;
            }
        }
    } finally {
        await fs.promises.rm(temp, { recursive: true, force: true });
    }
    return target;
}

/** A copy of this version still being made is never this old; an older one was cut short. */
const STALE_COPY_MS = 60 * 60 * 1000;

/**
 * Windows refuses to rename a folder while a virus scanner still holds a file just written into it (`EPERM`,
 * `EACCES`, `EBUSY`); as `graceful-fs` does, this tries again for a few seconds. It gives up at once when the target
 * is a complete copy already (another window's), which the caller accepts.
 */
async function renameRetrying(from: string, to: string): Promise<void> {
    for (let wait = 50; ; wait *= 2) {
        try {
            await fs.promises.rename(from, to);
            return;
        } catch (e) {
            const code = (e as NodeJS.ErrnoException).code;
            if (wait > 3200 || !(code === "EPERM" || code === "EACCES" || code === "EBUSY") || isComplete(to)) {
                throw e;
            }
            await new Promise((resolve) => setTimeout(resolve, wait));
        }
    }
}

/**
 * Removes what other versions left in `storage`: their copies, their lock files and their interrupted copies.
 * This version's interrupted copies go only when older than an hour: a younger one may be another window's copy
 * under way. Best effort: a folder another window still builds in is left for the next start.
 */
export async function removeOtherVersions(storage: string, version: string): Promise<void> {
    let names: string[];
    try {
        names = await fs.promises.readdir(storage);
    } catch {
        return; // no storage yet
    }
    const own = PREFIX + version;
    for (const name of names) {
        if (!name.startsWith(PREFIX) || name === own || name === `${own}.lock`) {
            continue;
        }
        const full = path.join(storage, name);
        try {
            if (name.startsWith(`${own}.tmp-`) && Date.now() - (await fs.promises.stat(full)).mtimeMs < STALE_COPY_MS) {
                continue;
            }
            await fs.promises.rm(full, { recursive: true, force: true });
        } catch {
            // In use by another window.
        }
    }
}
