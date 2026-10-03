// Entry point loaded by VS Code's extension host: runs every *.test.js next to this file with mocha.
import * as fs from "fs";
import * as path from "path";
import Mocha from "mocha";

export function run(): Promise<void> {
    const mocha = new Mocha({ ui: "bdd", color: true, timeout: 120000 });
    const grep = process.env.QB64RUST_TEST_GREP;
    if (grep) {
        mocha.grep(grep);
    }
    for (const name of fs.readdirSync(__dirname).sort()) {
        if (name.endsWith(".test.js")) {
            mocha.addFile(path.join(__dirname, name));
        }
    }
    return new Promise((resolve, reject) => {
        mocha.run((failures) => (failures > 0 ? reject(new Error(`${failures} test(s) failed`)) : resolve()));
    });
}
