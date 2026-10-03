// Turns the old compiler's streamed build output into lines for the output channel and progress for the status
// item. Pure: no `vscode` import.
import { PROGRESS } from "./parseQb64peOutput";

/** Where a `-x` build is: generating C++ (with the percentage of the bar), or compiling it. */
export type BuildProgress = { phase: "generating"; percent: number } | { phase: "compiling" };

const COMPILING = /^Compiling C\+\+ code/;

/**
 * Collects streamed output into whole lines. The `-x` progress bar redraws itself with `\r` on one line, about
 * 50 times per build; a line shows only its last redraw, and progress-bar lines are dropped. Each redraw of the
 * bar, and the start of the C++ step, is reported to `onProgress` as soon as it arrives.
 */
export class BuildOutputLines {
    /** Text since the last `\r` or `\n`. */
    private segment = "";
    /** Last non-empty redraw of the current line. */
    private lineLast = "";

    constructor(private readonly onProgress?: (progress: BuildProgress) => void) {}

    /** Adds a chunk; returns the lines it completed. */
    push(text: string): string[] {
        const lines: string[] = [];
        for (const part of text.split(/(\r|\n)/)) {
            if (part === "\r") {
                this.endSegment();
            } else if (part === "\n") {
                this.endSegment();
                lines.push(...this.endLine());
            } else {
                this.segment += part;
            }
        }
        return lines;
    }

    /**
     * The unfinished last line, if it has anything to show. Reports no progress: it is called after the run has
     * ended (a killed build can stop in the middle of the bar), and the status must not show it as still running.
     */
    flush(): string[] {
        this.endSegment(false);
        return this.lineLast ? this.endLine() : [];
    }

    private endSegment(report = true): void {
        const s = this.segment;
        this.segment = "";
        if (s === "") {
            return;
        }
        this.lineLast = s;
        if (!report) {
            return;
        }
        const t = s.trim();
        if (PROGRESS.test(t)) {
            this.onProgress?.({ phase: "generating", percent: Number(/(\d+)%$/.exec(t)![1]) });
        } else if (COMPILING.test(t)) {
            this.onProgress?.({ phase: "compiling" });
        }
    }

    private endLine(): string[] {
        const last = this.lineLast;
        this.lineLast = "";
        return PROGRESS.test(last.trim()) ? [] : [last];
    }
}

/** Status text for a build in progress. */
export function progressText(progress: BuildProgress): string {
    return progress.phase === "generating" ? `generating C++ ${progress.percent}%` : "compiling C++…";
}
