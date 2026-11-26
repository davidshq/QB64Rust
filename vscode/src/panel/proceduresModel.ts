// The rows of the SUBs and FUNCTIONs view (OpenSpec change editor-side-panel, design D3; spec editor/side-panel,
// "The SUBs and FUNCTIONs view"), from the answer of `qb64rust/procedures`. No VS Code import.
import type { ProceduresResult } from "../server/protocol";

export interface ProcedureRow {
    /** "Main program", or the procedure's name as declared with what its header has after it: `DrawBall (x, y)`. */
    label: string;
    /** The name alone, which A to Z sorts by. */
    name: string;
    /** The name of the row's file. */
    file: string;
    /** `SUB DrawBall (x, y)`, for the tooltip; the main program's file name for its row. */
    detail: string;
    kind: "main" | "SUB" | "FUNCTION";
    uri: string;
    /** 0-based, as LSP counts. */
    line: number;
}

/** The last part of a uri's path, decoded. */
export function fileName(uri: string): string {
    return decodeURIComponent(uri.substring(uri.lastIndexOf("/") + 1));
}

/** Whether the view is shown: the program has an include file. For a program of one file, VS Code's own Outline
 * lists the same procedures (user, 2026-11-26). */
export function hasIncludeFiles(result: ProceduresResult | undefined): boolean {
    return Boolean(result?.main) && (result?.files?.length ?? 0) > 1;
}

/** The main program's row first, then each procedure in the program's order, or A to Z when `byName`. Empty when the
 * server had no parse of the document. */
export function procedureRows(result: ProceduresResult | undefined, byName: boolean): ProcedureRow[] {
    if (!result?.main) {
        return [];
    }
    const main: ProcedureRow = { label: "Main program", name: "", file: fileName(result.main), detail: fileName(result.main), kind: "main", uri: result.main, line: 0 };
    const rows: ProcedureRow[] = result.procedures.map((p) => ({
        label: p.params ? `${p.name} ${p.params}` : p.name,
        name: p.name,
        file: fileName(p.uri),
        detail: `${p.kind} ${p.name}${p.params ? ` ${p.params}` : ""}`,
        kind: p.kind,
        uri: p.uri,
        line: p.range.start.line,
    }));
    if (byName) {
        rows.sort((a, b) => a.name.localeCompare(b.name, "en", { sensitivity: "base" }) || a.file.localeCompare(b.file));
    }
    return [main, ...rows];
}
