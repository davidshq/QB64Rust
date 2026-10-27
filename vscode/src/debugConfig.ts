// What a debug launch becomes (change m5-debug-launch, design D3): the rules, without `vscode`, so that the unit
// tests run them. `debug.ts` gathers the facts and acts on the result.
import * as path from "path";
import type { BuildWith } from "./config";

export const DEBUG_TYPE = "qb64rust";

/** A launch configuration of type `qb64rust` as `launch.json` or F5 gives it (spec editor/debug-launch). */
export interface LaunchConfig {
    type: string;
    request: string;
    name: string;
    program?: string;
    args?: string[];
    cwd?: string;
    stopOnEntry?: boolean;
    console?: "internalConsole" | "integratedTerminal" | "externalTerminal";
    noPrompt?: boolean;
    [key: string]: unknown;
}

/** Whether the `qb64rust` binary was found, and if not, why (as `LanguageServer.binary` says). */
export type Binary = { found: true } | { found: false; reason: string };

/** What the rules need to know of the editor and the settings. */
export interface Facts {
    trusted: boolean;
    /** The active editor's file when it is a `qb64rust` document on disk. */
    activeFile?: string;
    /** The first workspace folder, against which a relative `program` is resolved. */
    folder?: string;
    /** `qb64rust.runArguments`, as typed. */
    runArguments: string;
    /** `qb64rust.noPrompt`. */
    noPrompt: boolean;
    buildWith: BuildWith;
    binary: Binary;
}

/** Why there is no session, and the setting that would change it. */
export interface Refusal {
    kind: "refuse";
    message: string;
    setting?: string;
}

export type Launch =
    | Refusal
    /** `program` is an executable: launched as it is. */
    | { kind: "launch"; config: LaunchConfig }
    /** `program` is a source: built with `--debug`, then launched with the executable as `program`. */
    | { kind: "build"; source: string; config: LaunchConfig };

/** The configuration F5 debugs with no `launch.json`, and the one a new `launch.json` holds. */
export function initialConfiguration(): LaunchConfig {
    return { type: DEBUG_TYPE, request: "launch", name: "Debug QB64 program", program: "${file}" };
}

/** The configuration the Debug command starts for `file`. */
export function fileConfiguration(file: string): LaunchConfig {
    return { type: DEBUG_TYPE, request: "launch", name: `Debug ${path.basename(file)}`, program: file };
}

/** The adapter's arguments (spec editor/debug-launch, "The adapter"): `dap` with the toolchain folder a build uses
 * (change m6-toolchain, design D6). */
export function adapterArgs(toolchain: string): string[] {
    return ["dap", "--toolchain-root", toolchain];
}

/**
 * Why a debug build cannot be made, or undefined: debugging needs a build by `qb64rust` (design D4), so QB64pe
 * chosen or the binary not found is a refusal. Shared with `planBuild`.
 */
export function debugBuildRefusal(buildWith: BuildWith, binary: Binary): Refusal | undefined {
    if (buildWith === "qb64pe") {
        return { kind: "refuse", message: 'debugging needs a build by qb64rust, but qb64rust.buildWith is "qb64pe"', setting: "qb64rust.buildWith" };
    }
    if (!binary.found) {
        return { kind: "refuse", message: `debugging needs qb64rust, but ${binary.reason}`, setting: "qb64rust.path" };
    }
    return undefined;
}

/** The table of design D3, with the defaults of spec editor/debug-launch ("Launch defaults") filled in. */
export function resolveLaunch(config: LaunchConfig, facts: Facts): Launch {
    if (!facts.trusted) {
        return { kind: "refuse", message: "debugging requires a trusted workspace. Nothing was executed." };
    }
    let program = config.program ?? facts.activeFile;
    if (!program) {
        return { kind: "refuse", message: "open a QB64 program first." };
    }
    if (!path.isAbsolute(program) && facts.folder) {
        program = path.resolve(facts.folder, program);
    }
    const full: LaunchConfig = {
        ...config,
        program,
        args: config.args ?? splitArgs(facts.runArguments),
        cwd: config.cwd ?? path.dirname(program),
        stopOnEntry: config.stopOnEntry ?? false,
        console: config.console ?? "integratedTerminal",
        noPrompt: config.noPrompt ?? facts.noPrompt,
    };
    if (program.toLowerCase().endsWith(".exe")) {
        return { kind: "launch", config: full };
    }
    return debugBuildRefusal(facts.buildWith, facts.binary) ?? { kind: "build", source: program, config: full };
}

/**
 * Splits a command line into arguments as the C runtime of a Windows program does (what the program's `COMMAND$` is
 * made from): white space separates; double quotes group, and `""` inside them is a quote; backslashes are literal
 * except before a quote, where 2n of them give n and the quote groups, and 2n+1 give n and a literal quote.
 */
export function splitArgs(text: string): string[] {
    const args: string[] = [];
    let current = "";
    let inArg = false;
    let quoted = false;
    for (let i = 0; i < text.length; i++) {
        const c = text[i];
        if (c === "\\") {
            let n = 0;
            while (text[i + n] === "\\") {
                n++;
            }
            if (text[i + n] === '"') {
                current += "\\".repeat(Math.floor(n / 2));
                if (n % 2 === 1) {
                    current += '"';
                    i += n;
                } else {
                    i += n - 1; // the quote is read next
                }
            } else {
                current += "\\".repeat(n);
                i += n - 1;
            }
            inArg = true;
        } else if (c === '"') {
            if (quoted && text[i + 1] === '"') {
                current += '"';
                i++;
            } else {
                quoted = !quoted;
            }
            inArg = true;
        } else if ((c === " " || c === "\t") && !quoted) {
            if (inArg) {
                args.push(current);
                current = "";
                inArg = false;
            }
        } else {
            current += c;
            inArg = true;
        }
    }
    if (inArg) {
        args.push(current);
    }
    return args;
}
