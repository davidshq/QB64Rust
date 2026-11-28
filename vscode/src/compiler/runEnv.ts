// The environment a program is run with: `QB64PE_NOPROMPT` for `qb64rust.noPrompt`, and the four `QB64PE_LOG_*`
// variables for `qb64rust.run.log*`. The runtime reads them when the program starts (`logging.cpp`,
// `libqb_log_init`), so they work alike for a program built by either compiler. The QB64pe IDE sets the four the
// same way when it runs a program, and only when the level is not "None" (`qb64pe.bas`, "execute program").

/** The values of `qb64rust.run.log*`. */
export interface LogSettings {
    /** `none`, `trace`, `information`, `warning` or `error`. */
    level: string;
    scopes: readonly string[];
    handlers: readonly string[];
    /** The log file; empty is `qb64pe-log.txt` in the folder the program runs in. */
    file: string;
}

/** The log file's name when `qb64rust.run.logFile` is empty. The QB64pe IDE's default has the same name. */
export const DEFAULT_LOG_FILE = "qb64pe-log.txt";

/** The variables to add to the program's environment, or `undefined` when there are none. */
export function runEnvironment(noPrompt: boolean, log: LogSettings): Record<string, string> | undefined {
    const env: Record<string, string> = {};
    if (noPrompt) {
        env.QB64PE_NOPROMPT = "y";
    }
    if (log.level !== "none") {
        env.QB64PE_LOG_LEVEL = log.level;
        env.QB64PE_LOG_SCOPES = log.scopes.join(",");
        env.QB64PE_LOG_HANDLERS = log.handlers.join(",");
        // Always set: the runtime opens the file with this name when `file` is a handler, and has no default.
        env.QB64PE_LOG_FILE_PATH = log.file.trim() || DEFAULT_LOG_FILE;
    }
    return Object.keys(env).length ? env : undefined;
}
