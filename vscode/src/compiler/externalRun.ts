// Run in a console window of its own (`qb64rust.run.terminal` set to `external`), as the QB64pe IDE starts a
// program outside itself. Windows: `cmd /c start`, which gives a console program its own window. Linux: the
// terminal VS Code's `terminal.external.linuxExec` names, with `-e`, which most terminals take (not checked on every
// terminal; `x-terminal-emulator` when the setting is empty).

/** The process to spawn: its command, its arguments, and whether they are passed as written (Windows `cmd`). */
export interface ExternalStart {
    command: string;
    args: string[];
    verbatim: boolean;
}

/** How to start `exe` in folder `dir` with the arguments `args` as typed, in a window of its own. */
export function externalStart(platform: NodeJS.Platform, dir: string, exe: string, args: string, linuxExec: string): ExternalStart {
    const tail = args ? ` ${args}` : "";
    if (platform === "win32") {
        // `start`'s first quoted argument is the window's title. `cmd` expands `%NAME%` even inside quotes, so a
        // folder or an argument with `%` in it may reach the program changed.
        return { command: "cmd.exe", args: ["/d", "/c", `start "QB64" /D "${dir}" "${exe}"${tail}`], verbatim: true };
    }
    const q = (s: string) => `'${s.replace(/'/g, "'\\''")}'`;
    return { command: linuxExec.trim() || "x-terminal-emulator", args: ["-e", "sh", "-c", `cd ${q(dir)} && ${q(exe)}${tail}`], verbatim: false };
}
