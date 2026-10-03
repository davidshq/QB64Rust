// Runs compiler processes one at a time (design D4): all QB64pe runs share `<qb64pe>\internal\temp`. A new check
// of a file cancels the queued or running check of the same file; builds and formats are never cancelled by checks.
// No `vscode` import, so it is unit-tested with a fake process.
import { ChildProcess, spawn, spawnSync } from "child_process";

export type RunKind = "check" | "format" | "build";

export interface RunRequest {
    kind: RunKind;
    /** Checks with the same key replace each other (the file path). */
    key?: string;
    command: string;
    args: string[];
    cwd?: string;
    env?: NodeJS.ProcessEnv;
    /** Kill the process after this many milliseconds; none if undefined. */
    timeoutMs?: number;
    /** Receives output as it arrives (build output channel). */
    onOutput?: (text: string) => void;
}

export interface RunResult {
    exitCode: number;
    /** stdout and stderr, interleaved as received, decoded as Latin-1 (bytes preserved one to one). */
    output: string;
    cancelled: boolean;
    timedOut: boolean;
}

interface Job {
    request: RunRequest;
    resolve: (r: RunResult) => void;
    cancelled: boolean;
    child?: ChildProcess;
}

export class RunQueue {
    private readonly waiting: Job[] = [];
    private active: Job | undefined;
    private disposed = false;

    /** Called with the kind of the run that starts, or undefined when the queue becomes idle. */
    onBusyChange: ((kind: RunKind | undefined) => void) | undefined;

    run(request: RunRequest): Promise<RunResult> {
        if (request.kind === "check" && request.key !== undefined) {
            this.cancelChecks(request.key);
        }
        return new Promise((resolve) => {
            const job: Job = { request, resolve, cancelled: false };
            if (this.disposed) {
                resolve({ exitCode: -1, output: "", cancelled: true, timedOut: false });
                return;
            }
            this.waiting.push(job);
            this.next();
        });
    }

    /** Cancels queued and running checks of one file. */
    cancelChecks(key: string): void {
        for (let i = this.waiting.length - 1; i >= 0; i--) {
            const job = this.waiting[i];
            if (job.request.kind === "check" && job.request.key === key) {
                this.waiting.splice(i, 1);
                job.resolve({ exitCode: -1, output: "", cancelled: true, timedOut: false });
            }
        }
        const active = this.active;
        if (active && active.request.kind === "check" && active.request.key === key) {
            active.cancelled = true;
            killTree(active.child);
        }
    }

    get idle(): boolean {
        return this.active === undefined && this.waiting.length === 0;
    }

    private next(): void {
        if (this.active || this.waiting.length === 0) {
            if (!this.active) {
                this.onBusyChange?.(undefined);
            }
            return;
        }
        const job = this.waiting.shift()!;
        this.active = job;
        this.onBusyChange?.(job.request.kind);
        this.start(job);
    }

    private start(job: Job): void {
        const { request } = job;
        const chunks: string[] = [];
        let timedOut = false;
        let timer: NodeJS.Timeout | undefined;
        let finished = false;

        const finish = (exitCode: number) => {
            if (finished) {
                return;
            }
            finished = true;
            if (timer) {
                clearTimeout(timer);
            }
            this.active = undefined;
            job.resolve({ exitCode, output: chunks.join(""), cancelled: job.cancelled, timedOut });
            this.next();
        };

        let child: ChildProcess;
        try {
            child = spawn(request.command, request.args, {
                cwd: request.cwd,
                env: request.env ?? process.env,
                windowsHide: true,
                detached: process.platform !== "win32", // own process group, so the whole tree can be killed
            });
        } catch (err) {
            chunks.push(String(err));
            finish(-1);
            return;
        }
        job.child = child;
        const onData = (data: Buffer) => {
            const text = data.toString("latin1");
            chunks.push(text);
            request.onOutput?.(text);
        };
        child.stdout?.on("data", onData);
        child.stderr?.on("data", onData);
        child.on("error", (err) => {
            chunks.push(`${err.message}\n`);
            finish(-1);
        });
        child.on("close", (code) => finish(code ?? -1));

        if (request.timeoutMs !== undefined) {
            timer = setTimeout(() => {
                timedOut = true;
                killTree(child);
            }, request.timeoutMs);
        }
    }

    /** Cancels everything; the running process is killed. */
    dispose(): void {
        this.disposed = true;
        for (const job of this.waiting.splice(0)) {
            job.resolve({ exitCode: -1, output: "", cancelled: true, timedOut: false });
        }
        if (this.active) {
            this.active.cancelled = true;
            killTree(this.active.child);
        }
    }
}

/** Kills a process and its children (the compiler starts the C++ toolchain). */
export function killTree(child: ChildProcess | undefined): void {
    if (!child || child.pid === undefined || child.exitCode !== null) {
        return;
    }
    if (process.platform === "win32") {
        spawnSync("taskkill", ["/pid", String(child.pid), "/T", "/F"], { windowsHide: true });
    } else {
        try {
            process.kill(-child.pid, "SIGKILL");
        } catch {
            child.kill("SIGKILL");
        }
    }
}
