// Where browser.mjs looks for Chrome or Edge, given the platform and the environment, so that a test can check the
// list without a browser (test_find_chrome.py). Nothing here names a folder of one machine: on Windows the folders
// come from the environment, and elsewhere the names are looked up on PATH.
import { existsSync } from "node:fs";
import { posix, win32 } from "node:path";

// The names a Linux install of Chrome or Chromium puts on PATH, in the order Google's chrome-launcher tries them.
const UNIX_NAMES = ["google-chrome-stable", "google-chrome", "chromium-browser", "chromium"];

// The files to try, in order. On Windows: Chrome for all users (64-bit, then 32-bit), Chrome for one user (an
// install without admin rights goes to LOCALAPPDATA), then Edge, which every Windows 10 and 11 has. Elsewhere: each
// name in each folder of PATH (a Snap install's /snap/bin is one).
export function chromeCandidates(platform, env) {
  if (platform === "win32") {
    const programFiles = [env.PROGRAMFILES, env["PROGRAMFILES(X86)"]].filter(Boolean);
    const roots = [...programFiles, env.LOCALAPPDATA].filter(Boolean);
    const chrome = roots.map((r) => win32.join(r, "Google", "Chrome", "Application", "chrome.exe"));
    const edge = [env["PROGRAMFILES(X86)"], env.PROGRAMFILES]
      .filter(Boolean)
      .map((r) => win32.join(r, "Microsoft", "Edge", "Application", "msedge.exe"));
    return [...new Set([...chrome, ...edge])];
  }
  const folders = (env.PATH || "").split(posix.delimiter).filter(Boolean);
  return UNIX_NAMES.flatMap((name) => folders.map((f) => posix.join(f, name)));
}

// Windows keeps the names of environment variables in any letter case (ProgramFiles, PROGRAMFILES); Node's
// process.env finds them in any case, but a copy of it does not, so the names are made upper case first.
function upperKeys(env) {
  return Object.fromEntries(Object.entries(env).map(([k, v]) => [k.toUpperCase(), v]));
}

// The first candidate that exists, or null.
export function findInstalledChrome(platform = process.platform, env = process.env) {
  const e = platform === "win32" ? upperKeys(env) : env;
  return chromeCandidates(platform, e).find((c) => existsSync(c)) || null;
}
