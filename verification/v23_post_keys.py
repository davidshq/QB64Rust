"""Measurement for the screen-state check (OpenSpec change m2-screen-state-check, design D4).

Starts a QB64 program that has a window and posts keys to that window, without bringing it to the
foreground. This is the method that was measured and not taken: the keys are given from inside the
program instead (tools/screen/screen_state.cpp). Kept so that the measurement can be repeated.

    python v23_post_keys.py [--timeout S] <exe> <step>...

Steps:
    key:<name>    WM_KEYDOWN and WM_KEYUP of a key (enter, esc, up, f1, a to z, 0 to 9, ...)
    char:<text>   WM_CHAR for each character
    code:<n>      WM_CHAR with this code
    spare         WM_CHAR "~": after a special key the runtime ignores the next character
    focus         WM_SETFOCUS (the runtime drops keys while it believes the window has no focus)
    unfocus       WM_KILLFOCUS
    wait:<s>      sleep
    close         WM_CLOSE
    until:<name>  the key every 100 ms until the program exits

Found on 2026-10-10 with tests/corpus/screen/c05_inkey_keys.bas and c07_input_window.bas, built by
hand with tools/screen/screen_state.cpp on the link line so that the state shows the keys received:
    char:aab key:enter               97 97 98 13
    key:a key:enter                  13          (the letter's WM_CHAR comes too late)
    key:a wait:0.3 key:enter         97 13
    key:up char:Z key:enter          x72 13      (the character after a special key is ignored)
    key:up spare char:Z key:enter    x72 90 13
    code:27 code:9 code:13 ...       nothing     (GLFW drops WM_CHAR below 32)
    16 or more characters at once    the rest are lost (the INKEY$ buffer holds 15)
"""

from __future__ import annotations

import ctypes
import os
import subprocess
import sys
import time
from ctypes import wintypes

WM_SETFOCUS, WM_KILLFOCUS, WM_CLOSE = 0x0007, 0x0008, 0x0010
WM_KEYDOWN, WM_KEYUP, WM_CHAR = 0x0100, 0x0101, 0x0102
VK = {
    "enter": 0x0D,
    "esc": 0x1B,
    "space": 0x20,
    "tab": 0x09,
    "backspace": 0x08,
    "left": 0x25,
    "up": 0x26,
    "right": 0x27,
    "down": 0x28,
    "home": 0x24,
    "delete": 0x2E,
    "f1": 0x70,
    "f10": 0x79,
}
EXTENDED = {0x24, 0x25, 0x26, 0x27, 0x28, 0x2E}
ENUM = ctypes.WINFUNCTYPE(wintypes.BOOL, wintypes.HWND, wintypes.LPARAM)


def window_of(u32, pid: int) -> int | None:
    """The GLFW window of a process, or None."""
    found = []

    def each(hwnd, _):
        owner = wintypes.DWORD()
        u32.GetWindowThreadProcessId(hwnd, ctypes.byref(owner))
        name = ctypes.create_unicode_buffer(64)
        u32.GetClassNameW(hwnd, name, 64)
        if owner.value == pid and name.value == "GLFW30":
            found.append(hwnd)
        return True

    u32.EnumWindows(ENUM(each), 0)
    return found[0] if found else None


def vk_of(name: str) -> int:
    if name in VK:
        return VK[name]
    if len(name) == 1 and name.isalnum():
        return ord(name.upper())
    raise SystemExit(f"unknown key {name}")


def main() -> int:
    args = sys.argv[1:]
    timeout = 20.0
    if args[:1] == ["--timeout"]:
        timeout, args = float(args[1]), args[2:]
    if not args:
        print(__doc__, file=sys.stderr)
        return 2
    u32 = ctypes.WinDLL("user32", use_last_error=True)
    u32.PostMessageW.argtypes = [wintypes.HWND, wintypes.UINT, wintypes.WPARAM, wintypes.LPARAM]

    def post(message: int, wparam: int = 0, lparam: int = 0) -> None:
        u32.PostMessageW(hwnd, message, wparam, lparam)

    def press(vk: int) -> None:
        scan = u32.MapVirtualKeyW(vk, 0) | (0x100 if vk in EXTENDED else 0)
        post(WM_KEYDOWN, vk, 1 | (scan << 16))
        post(WM_KEYUP, vk, 1 | (scan << 16) | 0xC0000000)

    with subprocess.Popen([os.path.abspath(args[0])]) as p:
        start = time.time()
        hwnd = None
        while hwnd is None and p.poll() is None and time.time() - start < timeout:
            hwnd = window_of(u32, p.pid)
            time.sleep(0.02)
        for step in args[1:] if hwnd else []:
            if p.poll() is not None:
                break
            kind, _, arg = step.partition(":")
            if kind == "key":
                press(vk_of(arg))
            elif kind == "char":
                for ch in arg:
                    post(WM_CHAR, ord(ch), 1)
            elif kind == "code":
                post(WM_CHAR, int(arg), 1)
            elif kind == "spare":
                post(WM_CHAR, ord("~"), 1)
            elif kind == "focus":
                post(WM_SETFOCUS)
            elif kind == "unfocus":
                post(WM_KILLFOCUS)
            elif kind == "wait":
                time.sleep(float(arg))
            elif kind == "close":
                post(WM_CLOSE)
            elif kind == "until":
                while p.poll() is None and time.time() - start < timeout:
                    press(vk_of(arg))
                    time.sleep(0.1)
            else:
                raise SystemExit(f"unknown step {step}")
        try:
            code = p.wait(max(0.1, timeout - (time.time() - start)))
        except subprocess.TimeoutExpired:
            p.kill()
            print("timeout, killed")
            return 1
    print("exit", code, "window" if hwnd else "no window")
    return 0


if __name__ == "__main__":
    sys.exit(main())
