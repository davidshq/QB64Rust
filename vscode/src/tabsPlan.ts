// Tabs in a QB64 file, without VS Code: whether a text has any, and its lines with them turned into spaces. The
// QB64pe IDE turns every tab into spaces up to the next column that is a multiple of 4 when it loads a file
// (`ide_methods.bas`, "fix tabs"). Here a tab inside a string literal stays, since it is part of what the program
// prints, and so does a tab in a `DATA` statement, since an unquoted item is data too and `READ` would see it.

/** Whether the text has a tab character. */
export function hasTabs(text: string): boolean {
    return text.includes("\t");
}

/** The line with each tab outside a string literal turned into spaces up to the next column that is a multiple of
 * 4, columns counted from 0 in the line as it is being turned. A comment's tabs are turned too. From a `DATA`
 * statement's first word to the end of the line, every tab stays; the indent before it is turned. */
export function expandTabs(line: string): string {
    let out = "";
    let inString = false;
    let inComment = false;
    let inData = false;
    // At the start of a statement: at the line's start, after its line number, or after a `:`.
    let atStart = true;
    const lineNumber = /^[\t ]*\d+/.exec(line)?.[0].length ?? 0;
    for (let i = 0; i < line.length; i++) {
        const c = line[i];
        if (c === "\t" && !inString && !inData) {
            out += " ".repeat(4 - (out.length % 4));
            continue;
        }
        if (!inComment && !inData) {
            if (c === '"') {
                inString = !inString;
                atStart = false;
            } else if (!inString) {
                if (c === "'") {
                    inComment = true;
                } else if (c === ":") {
                    atStart = true;
                } else if (atStart && c !== " " && i >= lineNumber) {
                    inData = /^DATA\b/i.test(line.slice(i));
                    atStart = false;
                }
            }
        }
        out += c;
    }
    return out;
}

/** The lines of a text that change, with their new text: `[line number, new text]`. */
export function tabEdits(lines: readonly string[]): [number, string][] {
    const out: [number, string][] = [];
    lines.forEach((line, i) => {
        if (hasTabs(line)) {
            const expanded = expandTabs(line);
            if (expanded !== line) {
                out.push([i, expanded]);
            }
        }
    });
    return out;
}
