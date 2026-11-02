// The output of `qb64rust test --format json` read as events, and the filters of a run (change basic-unit-tests,
// design D6 and D7; spec editor/test-explorer). No `vscode` import, so it is unit-tested.

/** A test found: its file as `qb64rust test` was given it, its SUB and the SUB's line (1-based). */
export interface FoundTest {
    file: string;
    name: string;
    line: number;
}

export type Outcome = "pass" | "fail" | "error" | "timeout" | "build-error";

/** A test's result. `location`: where libqb named the failure (a failed `_ASSERT`, a runtime error). */
export interface TestResultEvent extends FoundTest {
    outcome: Outcome;
    message: string;
    location: { file: string; line: number } | null;
    output: string;
    ms: number;
}

export type TestEvent =
    | ({ event: "test" } & FoundTest)
    | ({ event: "result" } & TestResultEvent)
    | { event: "summary"; passed: number; failed: number; errors: number; timeouts: number }
    | { event: "error"; message: string };

const OUTCOMES: readonly string[] = ["pass", "fail", "error", "timeout", "build-error"];

/** One line of the output as an event; undefined for a line that is not one (blank, or not JSON). */
export function parseEvent(line: string): TestEvent | undefined {
    const text = line.trim();
    if (!text.startsWith("{")) {
        return undefined;
    }
    let v: Record<string, unknown>;
    try {
        v = JSON.parse(text) as Record<string, unknown>;
    } catch {
        return undefined;
    }
    const str = (k: string) => (typeof v[k] === "string" ? (v[k] as string) : "");
    const num = (k: string) => (typeof v[k] === "number" ? (v[k] as number) : 0);
    switch (v.event) {
        case "test":
            return { event: "test", file: str("file"), name: str("name"), line: num("line") };
        case "result": {
            const loc = v.location as { file?: unknown; line?: unknown } | null | undefined;
            const outcome = str("outcome");
            return {
                event: "result",
                file: str("file"),
                name: str("name"),
                line: num("line"),
                outcome: (OUTCOMES.includes(outcome) ? outcome : "error") as Outcome,
                message: str("message"),
                location: loc && typeof loc.file === "string" && typeof loc.line === "number" ? { file: loc.file, line: loc.line } : null,
                output: str("output"),
                ms: num("ms"),
            };
        }
        case "summary":
            return { event: "summary", passed: num("passed"), failed: num("failed"), errors: num("errors"), timeouts: num("timeouts") };
        case "error":
            return { event: "error", message: str("message") };
        default:
            return undefined;
    }
}

/** Reads the output as it comes, in chunks that may end inside a line; each complete line's event. */
export class JsonLines {
    private rest = "";

    push(chunk: string): TestEvent[] {
        const text = this.rest + chunk;
        const lines = text.split("\n");
        this.rest = lines.pop() ?? "";
        return lines.map(parseEvent).filter((e): e is TestEvent => e !== undefined);
    }

    /** The last line, when the output did not end with a line end. */
    flush(): TestEvent[] {
        const last = parseEvent(this.rest);
        this.rest = "";
        return last ? [last] : [];
    }
}

/** The tests a `--list --format json` output names. */
export function listedTests(output: string): FoundTest[] {
    return output
        .split("\n")
        .map(parseEvent)
        .filter((e): e is { event: "test" } & FoundTest => e?.event === "test")
        .map(({ file, name, line }) => ({ file, name, line }));
}

/** The key of a test: its file and SUB as `qb64rust test`'s `--filter` matches them, any case, `/` for `\`. */
export function testKey(file: string, name: string): string {
    return `${file}:${name}`.toLowerCase().replace(/\\/g, "/");
}

/** What the run is asked for: whole files, or single tests of a file. */
export interface Chosen {
    file: string;
    /** Undefined: every test of the file. */
    name?: string;
}

/**
 * The arguments of a run of the chosen tests: `--format json`, the files, and a `--filter` per chosen item when not
 * every chosen item is a whole file (a whole file then gets the filter `<file>:`, which every test of it holds).
 * `--filter` keeps the tests whose key contains the text, so a test whose name starts with another's runs too:
 * the caller marks only the tests it was asked for.
 */
export function runArgs(chosen: Chosen[]): string[] {
    const files = [...new Set(chosen.map((c) => c.file))];
    const args = ["test", "--format", "json", ...files];
    if (chosen.some((c) => c.name !== undefined)) {
        for (const c of chosen) {
            args.push("--filter", c.name === undefined ? `${c.file}:` : `${c.file}:${c.name}`);
        }
    }
    return args;
}

/** The Test Explorer's state of an outcome (design D7): an error, a timeout and a build error are errored. */
export function stateOf(outcome: Outcome): "passed" | "failed" | "errored" {
    return outcome === "pass" ? "passed" : outcome === "fail" ? "failed" : "errored";
}

/** The message shown at a result: a timeout and a build error say what they are. */
export function messageOf(r: TestResultEvent): string {
    switch (r.outcome) {
        case "timeout":
            return r.message || "timed out";
        case "build-error":
            return `The test file did not build:\n${r.message}`;
        default:
            return r.message;
    }
}
