// The column of a build's message in the editor (OpenSpec change utf8-source, design D5). The compiler gives a
// 1-based byte column. In a single-byte encoding such as cp437 that is the character column. In a UTF-8 file each
// character before it may be more than one byte, so the bytes are counted along the line.

/** Whether VS Code's encoding id names UTF-8 (`utf8`, `utf8bom`). */
export function isUtf8(encoding: string | undefined): boolean {
    return encoding !== undefined && encoding.toLowerCase().startsWith("utf8");
}

/**
 * The 0-based editor column (in UTF-16 units, as VS Code counts) of the 1-based byte column `byteColumn` on the
 * line `text`. With `utf8` false, each character is one byte.
 */
export function characterColumn(text: string, byteColumn: number, utf8: boolean): number {
    const target = Math.max(0, byteColumn - 1);
    if (!utf8) {
        return target;
    }
    let bytes = 0;
    let i = 0;
    while (i < text.length && bytes < target) {
        const cp = text.codePointAt(i) ?? 0;
        bytes += cp < 0x80 ? 1 : cp < 0x800 ? 2 : cp < 0x10000 ? 3 : 4;
        i += cp > 0xffff ? 2 : 1;
    }
    return i;
}
