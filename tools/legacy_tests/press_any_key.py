#!/usr/bin/env python3
"""Runs a QB64pe console program on Windows and answers its "Press any key to continue".

Usage: press_any_key.py <exe> [<stdin file>]
(exit code = the program's exit code; 125 if this helper fails)

On Windows, END in a $CONSOLE program prints "Press any key to continue" and waits for a key
event from the console input buffer (sub_end in libqb.cpp, func__getconsoleinput in
libqb/src/console.cpp). With stdin redirected from the null device or a pipe no event ever
arrives, so the program never exits. (On Linux and macOS it reads one byte from stdin and gets
end-of-file, so no helper is needed there.)

run_legacy_tests.py starts this helper with a console of its own (CREATE_NO_WINDOW) and stdout
and stderr redirected to the result file. The helper starts the program in that console with
stdin = CONIN$, stdout and stderr inherited, and writes a Shift key press into the console
input every 50 ms until the program exits. END empties the input buffer first and then waits,
so the next press ends the wait. Use it only for programs that do not read the keyboard
themselves (true for the corpus: no INKEY$, SLEEP or _KEYHIT).

With a second argument the program's stdin is that file instead of CONIN$ (a corpus program with a
<name>.stdin sidecar: INPUT and LINE INPUT of a $CONSOLE:ONLY program read standard input). The
key presses still go into the console the program shares with this helper.
"""

from __future__ import annotations

import ctypes
import msvcrt
import subprocess
import sys
import time
from ctypes import wintypes

GENERIC_READ = 0x80000000
GENERIC_WRITE = 0x40000000
FILE_SHARE_READ = 1
FILE_SHARE_WRITE = 2
OPEN_EXISTING = 3
INVALID_HANDLE_VALUE = wintypes.HANDLE(-1).value
KEY_EVENT = 1
VK_SHIFT = 0x10
SHIFT_SCAN_CODE = 0x2A


class KEY_EVENT_RECORD(ctypes.Structure):
    _fields_ = [
        ("bKeyDown", wintypes.BOOL),
        ("wRepeatCount", wintypes.WORD),
        ("wVirtualKeyCode", wintypes.WORD),
        ("wVirtualScanCode", wintypes.WORD),
        ("uChar", wintypes.WCHAR),
        ("dwControlKeyState", wintypes.DWORD),
    ]


class INPUT_RECORD_EVENT(ctypes.Union):
    _fields_ = [("KeyEvent", KEY_EVENT_RECORD), ("pad", ctypes.c_byte * 16)]


class INPUT_RECORD(ctypes.Structure):
    _fields_ = [("EventType", wintypes.WORD), ("Event", INPUT_RECORD_EVENT)]


def main() -> int:
    if len(sys.argv) not in (2, 3):
        print("usage: press_any_key.py <exe> [<stdin file>]", file=sys.stderr)
        return 125
    k32 = ctypes.WinDLL("kernel32", use_last_error=True)
    k32.CreateFileW.restype = wintypes.HANDLE
    k32.CreateFileW.argtypes = [
        wintypes.LPCWSTR,
        wintypes.DWORD,
        wintypes.DWORD,
        wintypes.LPVOID,
        wintypes.DWORD,
        wintypes.DWORD,
        wintypes.HANDLE,
    ]
    k32.WriteConsoleInputW.argtypes = [
        wintypes.HANDLE,
        ctypes.POINTER(INPUT_RECORD),
        wintypes.DWORD,
        ctypes.POINTER(wintypes.DWORD),
    ]
    conin = k32.CreateFileW(
        "CONIN$",
        GENERIC_READ | GENERIC_WRITE,
        FILE_SHARE_READ | FILE_SHARE_WRITE,
        None,
        OPEN_EXISTING,
        0,
        None,
    )
    if conin == INVALID_HANDLE_VALUE:
        print(
            f"press_any_key: cannot open CONIN$ (error {ctypes.get_last_error()}); "
            "the helper needs a console of its own",
            file=sys.stderr,
        )
        return 125

    records = (INPUT_RECORD * 2)()
    for rec, down in zip(records, (True, False)):
        rec.EventType = KEY_EVENT
        key = rec.Event.KeyEvent
        key.bKeyDown, key.wRepeatCount = down, 1
        key.wVirtualKeyCode, key.wVirtualScanCode = VK_SHIFT, SHIFT_SCAN_CODE

    # Popen duplicates the handle as the child's (inheritable) stdin; stdout and stderr are this
    # process's own, i.e. the result file.
    if len(sys.argv) == 3:
        with open(sys.argv[2], "rb") as stdin_file:
            p = subprocess.Popen([sys.argv[1]], stdin=stdin_file)
    else:
        stdin_fd = msvcrt.open_osfhandle(conin, 0)
        p = subprocess.Popen([sys.argv[1]], stdin=stdin_fd)
    written = wintypes.DWORD()
    while p.poll() is None:
        if not k32.WriteConsoleInputW(conin, records, 2, ctypes.byref(written)):
            print(
                f"press_any_key: WriteConsoleInputW failed (error {ctypes.get_last_error()})",
                file=sys.stderr,
            )
            p.kill()
            return 125
        time.sleep(0.05)
    return p.returncode


if __name__ == "__main__":
    sys.exit(main())
