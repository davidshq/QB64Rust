// Stand-in for the compiler in runQueue unit tests: node fake-process.js <label> <sleepMs> <exitCode>
// Prints "start <label>", waits, prints "end <label>" and exits with the code. A grandchild process is
// started so that tree killing can be observed (its pid is printed).
const [label, sleepMs, exitCode] = process.argv.slice(2);
const { spawn } = require("child_process");
const grandchild = spawn(process.execPath, ["-e", "setTimeout(() => {}, 60000)"], { stdio: "ignore" });
console.log(`start ${label} grandchild ${grandchild.pid}`);
setTimeout(() => {
    console.log(`end ${label}`);
    grandchild.kill();
    process.exit(Number(exitCode));
}, Number(sleepMs));
