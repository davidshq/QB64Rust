import * as assert from "assert";
import { characterColumn, isUtf8 } from "../../src/compiler/columns";

// OpenSpec change utf8-source, design D5 and task 5.2: a build's column counts bytes.
describe("characterColumn", () => {
    const line = 'PRINT "é": PRINT x(';

    it("a UTF-8 file: the column after an accent is one byte higher, and lands on the same character", () => {
        // The `(` is the 19th character. In UTF-8 the compiler reports byte 20, since `é` is two bytes.
        assert.strictEqual(characterColumn(line, 20, true), 18);
        assert.strictEqual(line[18], "(");
    });

    it("a single-byte file: the byte column is the character column", () => {
        assert.strictEqual(characterColumn(line, 19, false), 18);
    });

    it("a character of four bytes is two UTF-16 units", () => {
        const astral = 'PRINT "\u{1F600}" +';
        // `+` is at byte 14 (7 + 4 + 1 + 1 + 1); in the editor it is unit 11.
        assert.strictEqual(characterColumn(astral, 14, true), 11);
        assert.strictEqual(astral[11], "+");
    });

    it("isUtf8 knows VS Code's ids", () => {
        assert.ok(isUtf8("utf8") && isUtf8("utf8bom") && isUtf8("UTF8"));
        assert.ok(!isUtf8("cp437") && !isUtf8(undefined));
    });
});
