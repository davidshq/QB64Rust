// The state of the QB64 side panel's Program view and of the status bar items (OpenSpec change editor-side-panel,
// design D2): one function of four inputs, the program, what runs, the diagnostics of the program's files and
// workspace trust. No VS Code import, so it is unit-tested without it.

/** What runs now: a compiler run of the queue, the first build's runtime library, or a download. */
export type Activity =
    | { kind: "check" }
    | { kind: "format" }
    | { kind: "build" }
    | { kind: "firstBuild" }
    | { kind: "download"; what: string };

/** A diagnostic of one of the program's files. `line` counts from 1. */
export interface ProgramDiagnostic {
    uri: string;
    /** The file's name, for the status line. */
    file: string;
    line: number;
    message: string;
    severity: "error" | "warning";
}

export interface ProgramInputs {
    /** The program Run builds: its main file. Undefined while no QB64 file has been active in the window. */
    program?: { uri: string; name: string };
    /** What runs now; the most telling one when several do (see `activityText`). */
    activity?: Activity;
    /** The diagnostics of the program's files, in any order. */
    diagnostics: ProgramDiagnostic[];
    trusted: boolean;
    /** The reference page of the first error's statement, when it has one (`qb64rust/helpTopic`). */
    errorPage?: string;
    /** That page's title (`IF`), for the link "Help on IF"; without one the link says "Help". */
    errorPageTitle?: string;
}

/** The status line of a program. `long` marks work that takes a while (the first build, a download): the page
 * shows a moving bar with it. */
export type Status =
    | { kind: "busy"; text: string; long: boolean }
    | { kind: "ok"; warnings: number }
    | { kind: "error"; count: number; first: ProgramDiagnostic; page?: string; pageTitle?: string };

export type ProgramState =
    | { kind: "none"; trusted: boolean }
    | { kind: "program"; uri: string; name: string; trusted: boolean; status: Status };

/** The text of what runs, in plain words (spec editor/side-panel, "The first build says what it does"). */
export function activityText(a: Activity): string {
    switch (a.kind) {
        case "check":
            return "Checking the program…";
        case "format":
            return "Formatting…";
        case "build":
            return "Building…";
        case "firstBuild":
            return "Building the QB64 runtime. It can take about a minute, and happens only the first time.";
        case "download":
            return `Downloading the ${a.what}. This happens only once.`;
    }
}

/** The activity to show of several: a download, then the first build, then the queue's run. */
export function mainActivity(queue: Activity | undefined, firstBuild: boolean, download: string | undefined): Activity | undefined {
    if (download) {
        return { kind: "download", what: download };
    }
    if (firstBuild) {
        return { kind: "firstBuild" };
    }
    return queue;
}

/** The first error: the main file's before an include file's, then by file name, then by line. */
export function firstError(diagnostics: ProgramDiagnostic[], mainUri: string): ProgramDiagnostic | undefined {
    const errors = diagnostics.filter((d) => d.severity === "error");
    errors.sort((a, b) => {
        const am = a.uri === mainUri ? 0 : 1;
        const bm = b.uri === mainUri ? 0 : 1;
        return am - bm || a.file.localeCompare(b.file) || a.uri.localeCompare(b.uri) || a.line - b.line;
    });
    return errors[0];
}

export function programState(inputs: ProgramInputs): ProgramState {
    const p = inputs.program;
    if (!p) {
        return { kind: "none", trusted: inputs.trusted };
    }
    let status: Status;
    const first = firstError(inputs.diagnostics, p.uri);
    if (inputs.activity) {
        const long = inputs.activity.kind === "firstBuild" || inputs.activity.kind === "download";
        status = { kind: "busy", text: activityText(inputs.activity), long };
    } else if (first) {
        const count = inputs.diagnostics.filter((d) => d.severity === "error").length;
        status = { kind: "error", count, first, page: inputs.errorPage, pageTitle: inputs.errorPage ? inputs.errorPageTitle : undefined };
    } else {
        status = { kind: "ok", warnings: inputs.diagnostics.filter((d) => d.severity === "warning").length };
    }
    return { kind: "program", uri: p.uri, name: p.name, trusted: inputs.trusted, status };
}

/** The status bar's short form of the state (spec editor/side-panel, "The status bar item"). */
export function statusBarText(state: ProgramState): string | undefined {
    if (state.kind === "none") {
        return undefined;
    }
    const s = state.status;
    switch (s.kind) {
        case "busy":
            return "$(sync~spin) QB64";
        case "ok":
            return s.warnings ? `$(check) OK, ${plural(s.warnings, "warning")}` : "$(check) OK";
        case "error":
            return `$(error) ${plural(s.count, "error")}: line ${s.first.line}`;
    }
}

/** The status bar item's tooltip: the whole status line. */
export function statusBarTooltip(state: ProgramState): string | undefined {
    if (state.kind === "none") {
        return undefined;
    }
    const s = state.status;
    switch (s.kind) {
        case "busy":
            return s.text;
        case "ok":
            return `${state.name}: OK${s.warnings ? `, ${plural(s.warnings, "warning")}` : ""}. Click to open the QB64 side panel.`;
        case "error":
            return `${s.first.file}, line ${s.first.line}: ${s.first.message}. Click to open the QB64 side panel.`;
    }
}

export function plural(n: number, word: string): string {
    return `${n} ${word}${n === 1 ? "" : "s"}`;
}
