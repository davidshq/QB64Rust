import * as assert from "assert";
import * as fs from "fs";
import * as os from "os";
import * as path from "path";
import { copyVersion, libqbFolder, objectsLock, prepareLibqb, removeOtherVersions } from "../../src/compiler/libqbStore";

// Spec editor/packaging, "libqb objects outside the extension folder".
describe("libqbStore", () => {
    let dir: string;
    let source: string;
    let storage: string;

    beforeEach(() => {
        dir = fs.mkdtempSync(path.join(os.tmpdir(), "qb64rust-libqb-"));
        source = path.join(dir, "bundle", "libqb");
        fs.mkdirSync(path.join(source, "internal", "c"), { recursive: true });
        fs.writeFileSync(path.join(source, "internal", "c", "qbx.cpp"), "// qbx\n");
        fs.writeFileSync(path.join(source, "Makefile"), "all:\n");
        storage = path.join(dir, "storage");
    });

    afterEach(() => fs.rmSync(dir, { recursive: true, force: true }));

    it("copies the bundle into a folder named by the version and leaves the bundle alone", async () => {
        const target = await prepareLibqb(source, storage, "0.2.0");
        assert.strictEqual(target, libqbFolder(storage, "0.2.0"));
        assert.strictEqual(path.basename(target), "libqb-0.2.0");
        assert.ok(fs.existsSync(path.join(target, "internal", "c", "qbx.cpp")));
        assert.deepStrictEqual(fs.readdirSync(source).sort(), ["Makefile", "internal"]);
        assert.deepStrictEqual(fs.readdirSync(storage), ["libqb-0.2.0"]);
    });

    it("reuses a complete copy, with the objects a build wrote in it", async () => {
        const target = await prepareLibqb(source, storage, "0.2.0");
        fs.writeFileSync(path.join(target, "internal", "c", "libqb_make_000.o"), "obj");
        assert.strictEqual(await prepareLibqb(source, storage, "0.2.0"), target);
        assert.ok(fs.existsSync(path.join(target, "internal", "c", "libqb_make_000.o")));
    });

    it("makes an interrupted copy again", async () => {
        const target = libqbFolder(storage, "0.2.0");
        fs.mkdirSync(path.join(target, "internal"), { recursive: true }); // no qbx.cpp, no completion marker
        await prepareLibqb(source, storage, "0.2.0");
        assert.ok(fs.existsSync(path.join(target, "internal", "c", "qbx.cpp")));
    });

    it("fails when the bundle has no copy of libqb", async () => {
        await assert.rejects(prepareLibqb(path.join(dir, "nothing"), storage, "0.2.0"), /copy of libqb is missing/);
    });

    it("removes other versions' copies, locks and interrupted copies, and keeps this version's", async () => {
        await prepareLibqb(source, storage, "0.2.0");
        await prepareLibqb(source, storage, "0.2.1");
        fs.writeFileSync(objectsLock(storage, "0.2.0"), "");
        fs.writeFileSync(objectsLock(storage, "0.2.1"), "");
        fs.mkdirSync(path.join(storage, "libqb-0.2.0.tmp-1-ab"));
        fs.writeFileSync(path.join(storage, "other.json"), "{}");
        await removeOtherVersions(storage, "0.2.1");
        assert.deepStrictEqual(fs.readdirSync(storage).sort(), ["libqb-0.2.1", "libqb-0.2.1.lock", "other.json"]);
    });

    it("keeps this version's copy under way (another window's) and removes one cut short an hour ago", async () => {
        const recent = path.join(storage, "libqb-0.2.1.tmp-1-ab");
        const old = path.join(storage, "libqb-0.2.1.tmp-2-cd");
        fs.mkdirSync(recent, { recursive: true });
        fs.mkdirSync(old);
        const hourAgo = (Date.now() - 61 * 60 * 1000) / 1000;
        fs.utimesSync(old, hourAgo, hourAgo);
        await removeOtherVersions(storage, "0.2.1");
        assert.deepStrictEqual(fs.readdirSync(storage), ["libqb-0.2.1.tmp-1-ab"]);
    });

    it("names the copy by the version and the bundle's hash of libqb, or by the version alone without one", () => {
        assert.strictEqual(copyVersion("0.2.0", source), "0.2.0");
        fs.writeFileSync(path.join(dir, "bundle", "libqb.id"), "0123abcd89ef4567\n");
        assert.strictEqual(copyVersion("0.2.0", source), "0.2.0-0123abcd89ef4567");
        assert.strictEqual(path.basename(libqbFolder(storage, copyVersion("0.2.0", source))), "libqb-0.2.0-0123abcd89ef4567");
    });

    it("does nothing when there is no storage yet", async () => {
        await removeOtherVersions(storage, "0.2.0");
        assert.ok(!fs.existsSync(storage));
    });
});
