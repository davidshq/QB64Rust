import * as assert from "assert";
import { findCompiler } from "../../src/compiler/discovery";

const files = (...paths: string[]) => (p: string) => paths.includes(p);

describe("findCompiler", () => {
    it("setting wins over PATH", () => {
        const r = findCompiler({
            setting: "C:\\qb\\qb64pe.exe",
            envPath: "C:\\other",
            platform: "win32",
            isFile: files("C:\\qb\\qb64pe.exe", "C:\\other\\qb64pe.exe"),
        });
        assert.deepStrictEqual(r, { found: true, path: "C:\\qb\\qb64pe.exe", source: "setting" });
    });

    it("setting may name the installation folder", () => {
        const r = findCompiler({ setting: "C:\\qb", envPath: "", platform: "win32", isFile: files("C:\\qb\\qb64pe.exe") });
        assert.deepStrictEqual(r, { found: true, path: "C:\\qb\\qb64pe.exe", source: "setting" });
    });

    it("relative setting resolves against the workspace folder", () => {
        const r = findCompiler({ setting: "tools/qb64pe", baseDir: "/home/u/proj", envPath: "", platform: "linux", isFile: files("/home/u/proj/tools/qb64pe") });
        assert.deepStrictEqual(r, { found: true, path: "/home/u/proj/tools/qb64pe", source: "setting" });
    });

    it("missing configured path is reported, PATH is not used instead", () => {
        const r = findCompiler({ setting: "C:\\nope\\qb64pe.exe", envPath: "C:\\other", platform: "win32", isFile: files("C:\\other\\qb64pe.exe") });
        assert.strictEqual(r.found, false);
        assert.match(r.found ? "" : r.reason, /does not exist/);
    });

    it("PATH is searched in order (Windows separators and quotes)", () => {
        const r = findCompiler({ setting: "", envPath: 'C:\\a;"C:\\b";C:\\c', platform: "win32", isFile: files("C:\\b\\qb64pe.exe", "C:\\c\\qb64pe.exe") });
        assert.deepStrictEqual(r, { found: true, path: "C:\\b\\qb64pe.exe", source: "PATH" });
    });

    it("PATH on other platforms uses ':' and no .exe", () => {
        const r = findCompiler({ setting: "", envPath: "/usr/bin:/opt/qb64pe", platform: "linux", isFile: files("/opt/qb64pe/qb64pe") });
        assert.deepStrictEqual(r, { found: true, path: "/opt/qb64pe/qb64pe", source: "PATH" });
    });

    it("not found anywhere", () => {
        const r = findCompiler({ setting: "", envPath: "C:\\a", platform: "win32", isFile: () => false });
        assert.strictEqual(r.found, false);
    });
});
