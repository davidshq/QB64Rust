// Runs a window program built for the wasm target in headless Chrome, as a test runs a desktop program
// (OpenSpec change wasm-window). See README.md beside this file.
//
//   node browser.mjs <name>.html [--for <seconds>] [--screenshot <file.png>] [--chrome <chrome>] [--no-headers]
//                    [--env NAME=VALUE]... [--press <seconds>:<key>]... [--gesture-audio] [--save <file>]...
//   node browser.mjs --find-chrome
//
// The page's folder is the program's: Chrome is driven over --remote-debugging-pipe, so no port is opened. The
// folder is served at http://localhost/ through DevTools' request interception, with the two headers that a
// page with threads needs (Cross-Origin-Opener-Policy and Cross-Origin-Embedder-Policy), so nothing listens.
//
// Before the program starts, the folder's other files are written into the page's file system, in its current
// folder. The variables a test sets for a program (QB64_*, QB64PE_*, QB64RUST_*: the screen state's, the prompt
// setting, the trace) are given to it. One that names a file of the host names a file of the page instead: the
// host's file is written there first when it exists (QB64_SCREEN_KEYS), and when the program ends what the page's
// file holds is written to the host's (QB64_SCREEN_STATE, the trace). The program's standard output and error are
// printed, and this process exits with its exit code.
import { spawn } from "node:child_process";
import { appendFileSync, existsSync, mkdtempSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, extname, isAbsolute, join, resolve } from "node:path";
import { findInstalledChrome } from "./find_chrome.mjs";

// The key event of a name for --press: a name of KeyboardEvent.key (F1 to F12, Enter, Escape, Tab, Backspace,
// Space, the arrows, PageUp, PageDown, Home, End), or one printable character. The virtual key code is what GLFW
// reads. Null for a name it does not know.
function keyEvent(name) {
  const named = {
    Enter: [13, "\r"], Escape: [27], Tab: [9], Backspace: [8], Space: [32, " "],
    ArrowLeft: [37], ArrowUp: [38], ArrowRight: [39], ArrowDown: [40],
    PageUp: [33], PageDown: [34], End: [35], Home: [36],
  };
  for (let n = 1; n <= 12; n++) named[`F${n}`] = [111 + n];
  if (named[name]) {
    const [code, text] = named[name];
    const key = name === "Space" ? " " : name;
    return { key, code: name === "Space" ? "Space" : name, windowsVirtualKeyCode: code, text };
  }
  if (name.length !== 1 || name < " " || name > "~") return null;
  const upper = name.toUpperCase();
  const letter = upper >= "A" && upper <= "Z";
  const digit = name >= "0" && name <= "9";
  return {
    key: name,
    code: letter ? `Key${upper}` : digit ? `Digit${name}` : "",
    windowsVirtualKeyCode: letter || digit ? upper.charCodeAt(0) : 0,
    text: name,
  };
}

const args = process.argv.slice(2);
let page = null;
let screenshot = null;
let seconds = null;
let profileArg = null;
let isolation = true;
let chromeArg = process.env.QB64RUST_CHROME || null;
const extraEnv = []; // --env NAME=VALUE, in order
const presses = []; // --press <seconds>:<key>, in order
const saves = []; // --save <file>: files of the program's folder in the page, written beside the page at the end
let gestureAudio = false;
for (let i = 0; i < args.length; i++) {
  if (args[i] === "--screenshot") screenshot = args[++i];
  else if (args[i] === "--chrome") chromeArg = args[++i];
  else if (args[i] === "--for") seconds = Number(args[++i]);
  else if (args[i] === "--profile") profileArg = args[++i];
  else if (args[i] === "--no-headers") isolation = false;
  else if (args[i] === "--find-chrome") page = false;
  else if (args[i] === "--gesture-audio") gestureAudio = true;
  else if (args[i] === "--save") {
    const name = args[++i] || "";
    if (!name || isAbsolute(name) || name.split(/[\\/]/).includes("..")) {
      console.error(`browser.mjs: --save needs a file of the program's folder, not ${JSON.stringify(name)}`);
      process.exit(2);
    }
    saves.push(name.replace(/\\/g, "/"));
  }
  else if (args[i] === "--env") {
    const given = args[++i] || "";
    const eq = given.indexOf("=");
    if (eq <= 0) {
      console.error(`browser.mjs: --env needs NAME=VALUE, not ${JSON.stringify(given)}`);
      process.exit(2);
    }
    extraEnv.push([given.slice(0, eq), given.slice(eq + 1)]);
  } else if (args[i] === "--press") {
    const given = args[++i] || "";
    const colon = given.indexOf(":");
    const wait = Number(given.slice(0, colon));
    const key = given.slice(colon + 1);
    if (colon <= 0 || !(wait >= 0) || !keyEvent(key)) {
      console.error(`browser.mjs: --press needs <seconds>:<key>, not ${JSON.stringify(given)}`);
      process.exit(2);
    }
    presses.push([wait, key]);
  } else page = args[i];
}
// --find-chrome: prints the Chrome a run would use and exits 0, or exits 2 when there is none. The test runner
// asks this once, so that where there is no Chrome (the suites VM) it skips a window program instead of failing it.
if (page === false) {
  const found = findChrome();
  if (found) console.log(found);
  process.exit(found ? 0 : 2);
}
if (!page) {
  console.error(
    "usage: node browser.mjs <name>.html [--for <seconds>] [--screenshot <file.png>] [--profile <folder>] [--chrome <chrome>] [--no-headers]\n" +
      "         [--env NAME=VALUE]... [--press <seconds>:<key>]... [--gesture-audio] [--save <file>]...",
  );
  process.exit(2);
}
const folder = dirname(resolve(page));
const pageName = basename(page);
const script = pageName.replace(/\.html$/, ".js");
// A web folder (OpenSpec change wasm-publish): its page is index.html, and every file of it is the host's to serve,
// none the program's. Any other page is served with its script, its module and its data file.
const webFolder = pageName === "index.html";
const served = webFolder
  ? new Set(readdirSync(folder).filter((name) => statSync(join(folder, name)).isFile()))
  : new Set([pageName, script, pageName.replace(/\.html$/, ".wasm"), pageName.replace(/\.html$/, ".data")]);

// The launcher's own messages go to standard error, and with QB64RUST_BROWSER_LOG to that file too, one line each
// with the time and the page, with each run's exit status: a test runner may overwrite the output of a run.
const logFile = process.env.QB64RUST_BROWSER_LOG || null;
function note(text) {
  process.stderr.write(`browser.mjs: ${text}\n`);
  if (logFile) appendFileSync(logFile, `${new Date().toISOString()} ${pageName}: ${text}\n`);
}
process.on("unhandledRejection", (e) => {
  note(`unhandled: ${(e && e.stack) || e}`);
  process.exit(1);
});
process.on("uncaughtException", (e) => {
  note(`uncaught: ${(e && e.stack) || e}`);
  process.exit(1);
});

// Chrome: --chrome, QB64RUST_CHROME, or where an install puts it (find_chrome.mjs).
function findChrome() {
  return chromeArg || findInstalledChrome();
}
const chrome = findChrome();
if (!chrome) {
  console.error("browser.mjs: no Chrome found; name one with --chrome or QB64RUST_CHROME");
  process.exit(2);
}

// What the page's file system gets: every file of the folder but the page's own, and the files the variables name.
const files = [];
function collect(dir, prefix) {
  for (const name of readdirSync(dir)) {
    const path = join(dir, name);
    const rel = prefix + name;
    if (statSync(path).isDirectory()) collect(path, rel + "/");
    else if (!served.has(rel)) files.push([rel, readFileSync(path).toString("base64")]);
  }
}
if (!webFolder) collect(folder, "");
// The tools' own settings are not the program's.
const TOOL_VARIABLES = new Set([
  "QB64RUST_CHROME",
  "QB64RUST_EMSDK_ROOT",
  "QB64RUST_LIBQB_WASM_ROOT",
  "QB64RUST_ALLOW_NETWORK",
  "QB64RUST_BROWSER_LOG",
]);
const env = {};
const mapped = []; // [page's file, host's file, what it held when written, or null]
// The program's variables of this process, then those of --env, which may have any name (a game's own, such as
// CWS_SEED) and win over this process's.
const programEnv = Object.entries(process.env).filter(
  ([name]) => /^QB64(PE|RUST)?_/.test(name) && !TOOL_VARIABLES.has(name) && !extraEnv.some(([n]) => n === name),
);
for (const [name, value] of [...programEnv, ...extraEnv]) {
  if (isAbsolute(value)) {
    const inPage = `qb64rust-env/${name}`;
    let held = null;
    if (existsSync(value) && statSync(value).isFile()) {
      held = readFileSync(value).toString("base64");
      files.push([inPage, held]);
    }
    mapped.push([inPage, value, held]);
    env[name] = inPage;
  } else {
    env[name] = value;
  }
}

// The program's standard input: this process's, when it is not a terminal (a test's .stdin), read to its end.
let stdin = "";
if (!process.stdin.isTTY) {
  try {
    stdin = readFileSync(0).toString("base64");
  } catch {}
}

// Runs in the page before its own script: the Module the page's script keeps (see the driver's page).
const setup = `
(() => {
  const files = ${JSON.stringify(files)};
  const env = ${JSON.stringify(env)};
  const input = Uint8Array.from(atob(${JSON.stringify(stdin)}), (c) => c.charCodeAt(0));
  let read = 0;
  const send = (kind, value) => self.qb64rustReport && self.qb64rustReport(JSON.stringify([kind, value]));
  // Output byte by byte, sent at each line end, when 50 bytes wait, and before the end.
  const waiting = { out: [], err: [] };
  const flush = (kind) => {
    if (waiting[kind].length) {
      send(kind, btoa(String.fromCharCode.apply(null, waiting[kind])));
      waiting[kind] = [];
    }
  };
  const put = (kind) => (byte) => {
    if (byte === null || byte === undefined) return;
    waiting[kind].push(byte & 255);
    if (byte === 10 || waiting[kind].length >= 50) flush(kind);
  };
  setInterval(() => {
    flush("out");
    flush("err");
  }, 50);
  self.Module = {
    qb64rustKeep: ${JSON.stringify(profileArg !== null)},
    preRun: [() => {
      Module.FS.init(() => (read < input.length ? input[read++] : null), put("out"), put("err"));
      // The folder of the files that variables name: a file the program only writes (QB64_SCREEN_STATE) needs it.
      Module.FS.mkdirTree("qb64rust-env");
      for (const [name, data] of files) {
        const slash = name.lastIndexOf("/");
        if (slash > 0) Module.FS.mkdirTree(name.slice(0, slash));
        Module.FS.writeFile(name, Uint8Array.from(atob(data), (c) => c.charCodeAt(0)));
      }
      Object.assign(Module.ENV, env);
    }],
    print: (text) => send("line", text),
    printErr: (text) => send("errline", text),
    onExit: (code) => {
      flush("out");
      flush("err");
      send("exit", code);
    },
    onAbort: (what) => send("abort", String(what)),
  };
  // Each key pressed, and whether the page blocked the browser's own action for it, once every listener has run.
  self.qb64rustKeys = [];
  addEventListener("keydown", (event) => setTimeout(() => self.qb64rustKeys.push([event.key, event.defaultPrevented]), 0));
  self.qb64rustAudio = () => {
    const audio = self.miniaudio;
    const states = audio && audio.devices ? audio.devices.filter(Boolean).map((d) => (d.webaudio ? d.webaudio.state : "none")) : [];
    return states.length ? states.join(",") : "none";
  };
  self.qb64rustFile = (name) => {
    try {
      const data = Module.FS.readFile(name);
      let text = "";
      for (let i = 0; i < data.length; i += 0x8000) text += String.fromCharCode.apply(null, data.subarray(i, i + 0x8000));
      return btoa(text);
    } catch (e) {
      return null;
    }
  };
})();
`;

// A profile of this run's own, removed at the end; or --profile's, kept, with what the page keeps in it.
const profile = profileArg !== null ? resolve(profileArg) : mkdtempSync(join(tmpdir(), "qb64rust-chrome-"));
const proc = spawn(
  chrome,
  [
    "--headless=new",
    "--remote-debugging-pipe",
    `--user-data-dir=${profile}`,
    "--no-first-run",
    "--no-default-browser-check",
    "--disable-extensions",
    "--disable-background-networking",
    "--disable-component-update",
    "--disable-sync",
    "--mute-audio",
    // --gesture-audio: sound waits for the player's first action, as in a browser a player uses.
    `--autoplay-policy=${gestureAudio ? "document-user-activation-required" : "no-user-gesture-required"}`,
    "--use-angle=swiftshader",
    "--enable-unsafe-swiftshader",
    "--window-size=1280,1024",
    "about:blank",
  ],
  { stdio: ["ignore", "ignore", "ignore", "pipe", "pipe"] },
);
const toChrome = proc.stdio[3];
const fromChrome = proc.stdio[4];

let nextId = 1;
const pending = new Map();
function send(method, params = {}, sessionId) {
  const id = nextId++;
  const message = { id, method, params };
  if (sessionId) message.sessionId = sessionId;
  toChrome.write(JSON.stringify(message) + "\0");
  return new Promise((ok, fail) => pending.set(id, { ok, fail, method }));
}

let finished = false;
async function finish(code) {
  if (finished) return;
  finished = true;
  try {
    // A page that never started its program says why in its status line (a web folder's page on a host without
    // the two headers), or still says "Loading...".
    if (code === 124) {
      const expression = `(document.getElementById("qb64rust-status") || {}).textContent || ""`;
      const status = await send("Runtime.evaluate", { expression, returnByValue: true }, session);
      if (status.result.value) note(`the page says: ${status.result.value}`);
    }
    if (screenshot) {
      const shot = await send("Page.captureScreenshot", { format: "png" }, session);
      writeFileSync(screenshot, Buffer.from(shot.data, "base64"));
    }
    for (const [inPage, host, held] of mapped) {
      const expression = `self.qb64rustFile(${JSON.stringify(inPage)})`;
      const got = await send("Runtime.evaluate", { expression, returnByValue: true }, session);
      if (got.result.value !== null && got.result.value !== undefined && got.result.value !== held) {
        writeFileSync(host, Buffer.from(got.result.value, "base64"));
      }
    }
    // --save: a file the program wrote, such as a game's result, written into the page's folder on the host.
    for (const name of saves) {
      const expression = `self.qb64rustFile(${JSON.stringify(name)})`;
      const got = await send("Runtime.evaluate", { expression, returnByValue: true }, session);
      if (got.result.value === null || got.result.value === undefined) note(`--save ${name}: the program wrote no such file`);
      else writeFileSync(join(folder, name), Buffer.from(got.result.value, "base64"));
    }
  } catch (e) {
    note(e.message);
  }
  if (logFile) appendFileSync(logFile, `${new Date().toISOString()} ${pageName}: exit ${code}\n`);
  // With --profile the page keeps what the program wrote, and saves it when the program ends: give that a second.
  if (profileArg !== null) await new Promise((done) => setTimeout(done, 1000));
  await send("Browser.close").catch(() => {});
  setTimeout(() => {
    try {
      if (profileArg === null) rmSync(profile, { recursive: true, force: true });
    } catch {}
    process.exit(code);
  }, 300);
}

const types = { ".html": "text/html", ".js": "text/javascript", ".wasm": "application/wasm" };
// --no-headers: as a host that cannot send the two headers of threads, where a web folder's page gets them from
// its service worker.
const headers = [
  ...(isolation
    ? [
        { name: "Cross-Origin-Opener-Policy", value: "same-origin" },
        { name: "Cross-Origin-Embedder-Policy", value: "require-corp" },
        { name: "Cross-Origin-Resource-Policy", value: "same-origin" },
      ]
    : []),
  { name: "Cache-Control", value: "no-store" },
];
async function serve(params, sessionId) {
  const name = decodeURIComponent(new URL(params.request.url).pathname).replace(/^\/+/, "");
  const path = join(folder, name);
  if (served.has(name) && existsSync(path)) {
    await send(
      "Fetch.fulfillRequest",
      {
        requestId: params.requestId,
        responseCode: 200,
        responseHeaders: [...headers, { name: "Content-Type", value: types[extname(name)] || "application/octet-stream" }],
        body: readFileSync(path).toString("base64"),
      },
      sessionId,
    );
  } else {
    await send("Fetch.fulfillRequest", { requestId: params.requestId, responseCode: 404, responseHeaders: headers, body: "" }, sessionId);
  }
}

function event(message) {
  const { method, params, sessionId } = message;
  if (method === "Fetch.requestPaused") {
    serve(params, sessionId).catch((e) => note(e.message));
  } else if (method === "Runtime.bindingCalled" && params.name === "qb64rustReport") {
    const [kind, value] = JSON.parse(params.payload);
    if (kind === "out") process.stdout.write(Buffer.from(value, "base64"));
    else if (kind === "err") process.stderr.write(Buffer.from(value, "base64"));
    // What the runtime itself prints, a line at a time (the program's own output comes as bytes, above); Emscripten's
    // GL emulation says on every start that it is one, which is the SDK's, not the program's.
    else if (kind === "line") process.stdout.write(value + "\n");
    else if (kind === "errline" && !value.startsWith("WARNING: using emscripten GL ")) process.stderr.write(value + "\n");
    else if (kind === "exit") finish(value);
    else if (kind === "abort") {
      note(`the program aborted: ${value}`);
      finish(134);
    }
  } else if (method === "Page.javascriptDialogOpening") {
    // libqb's messages are the page's dialogs (an unhandled error's "Continue?"). Nobody answers them, as nobody
    // answers the dialog of a desktop program under test: a question is answered No, an alert closed.
    send("Page.handleJavaScriptDialog", { accept: params.type === "alert" }, sessionId).catch(() => {});
  } else if (method === "Runtime.exceptionThrown") {
    const d = params.exceptionDetails;
    note(d.exception ? d.exception.description : d.text);
  } else if (method === "Target.attachedToTarget") {
    // A thread's worker: its exceptions are reported with their stack, as the page's are.
    send("Runtime.enable", {}, params.sessionId).catch(() => {});
    send("Runtime.runIfWaitingForDebugger", {}, params.sessionId).catch(() => {});
  }
}

let buffer = "";
fromChrome.on("data", (chunk) => {
  buffer += chunk.toString("utf8");
  for (let end; (end = buffer.indexOf("\0")) >= 0; ) {
    const message = JSON.parse(buffer.slice(0, end));
    buffer = buffer.slice(end + 1);
    if (message.id && pending.has(message.id)) {
      const p = pending.get(message.id);
      pending.delete(message.id);
      if (message.error) p.fail(new Error(`${p.method}: ${message.error.message}`));
      else p.ok(message.result);
    } else if (message.method) {
      event(message);
    }
  }
});
proc.on("exit", () => {
  if (!finished) {
    note("Chrome ended before the program");
    process.exit(1);
  }
});

const { targetId } = await send("Target.createTarget", { url: "about:blank" });
const { sessionId: session } = await send("Target.attachToTarget", { targetId, flatten: true });
// With --no-headers the requests of the page's service worker are served too, so the whole browser's are taken,
// not only the page's.
await send("Fetch.enable", { patterns: [{ urlPattern: "http://localhost/*" }] }, isolation ? session : undefined);
await send("Runtime.enable", {}, session);
await send("Runtime.addBinding", { name: "qb64rustReport" }, session);
await send("Page.enable", {}, session);
await send("Page.addScriptToEvaluateOnNewDocument", { source: setup }, session);
await send("Target.setAutoAttach", { autoAttach: true, waitForDebuggerOnStart: false, flatten: true }, session);
await send("Page.navigate", { url: `http://localhost/${encodeURIComponent(pageName)}` }, session);
// --for: a program that has not ended by then is stopped, with the exit status of `timeout`, after the screenshot.
if (seconds !== null) setTimeout(() => finish(124), seconds * 1000);

// --press: each key after its wait, counted from the key before (the first from the page's start), as a player's
// key press. Each is noted with whether the page blocked the browser's own action for it, and the audio's state.
async function evaluate(expression) {
  const got = await send("Runtime.evaluate", { expression, returnByValue: true }, session);
  return got.result.value;
}
async function pressKeys() {
  for (const [index, [wait, name]] of presses.entries()) {
    await new Promise((done) => setTimeout(done, wait * 1000));
    if (finished) return;
    if (index === 0) note(`audio before the first key: ${await evaluate("self.qb64rustAudio()")}`);    const { text, ...key } = keyEvent(name);
    await send("Input.dispatchKeyEvent", { type: text ? "keyDown" : "rawKeyDown", ...key, ...(text ? { text, unmodifiedText: text } : {}) }, session);
    await send("Input.dispatchKeyEvent", { type: "keyUp", ...key }, session);
    await new Promise((done) => setTimeout(done, 300));
    if (finished) return;
    const seen = await evaluate("JSON.stringify(self.qb64rustKeys.shift() || null)");
    const [, blocked] = JSON.parse(seen) || [null, null];
    const action = blocked === null ? "not seen by the page" : blocked ? "blocked" : "not blocked";
    note(`pressed ${name}: ${action}, audio ${await evaluate("self.qb64rustAudio()")}`);
  }
}
if (presses.length) pressKeys().catch((e) => note(e.message));
