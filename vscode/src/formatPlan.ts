// Which formatter formats a document, and the lines of a selection (change m5b-formatter, design D7; spec
// editor/formatting, "Which formatter" and "Format selection"). No VS Code import, so it is unit-tested without it.
import type { BuildWith } from "./config";

export type FormatterChoice = "server" | "qb64rust" | "qb64pe";

/** The server while it runs; else `qb64rust -y` when the binary is found and the build is not QB64pe's; else
 * `qb64pe -y`. A failure of the one chosen is not retried with another. */
export function chooseFormatter(serverRunning: boolean, binaryFound: boolean, buildWith: BuildWith): FormatterChoice {
    if (serverRunning) {
        return "server";
    }
    return binaryFound && buildWith !== "qb64pe" ? "qb64rust" : "qb64pe";
}

/** A line of the document that the selection changes: its number and its formatted text. */
export interface LineEdit {
    line: number;
    text: string;
}

/**
 * The lines from `startLine` to `endLine` that differ between the document and its formatted text, both as lines
 * without their line ends, a line laid out as in the whole document. A selection that ends at the start of a line
 * (`endCharacter` 0) does not touch that line, as the server's range formatting.
 */
export function selectionEdits(lines: string[], formatted: string[], startLine: number, endLine: number, endCharacter: number): LineEdit[] {
    const last = endCharacter === 0 && endLine > startLine ? endLine - 1 : endLine;
    const edits: LineEdit[] = [];
    for (let line = startLine; line <= last; line++) {
        const old = lines[line];
        const text = formatted[line];
        if (old !== undefined && text !== undefined && old !== text) {
            edits.push({ line, text });
        }
    }
    return edits;
}

/** The lines of a text, line ends CR LF, LF or CR removed; no line after a last line end. */
export function splitLines(text: string): string[] {
    const lines = text.split(/\r\n|\n|\r/);
    if (lines.length > 1 && lines[lines.length - 1] === "") {
        lines.pop();
    }
    return lines;
}
