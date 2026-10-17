#!/usr/bin/env python3
"""Runs a QB64pe console program on Windows and answers its "Press any key to continue".

Usage: press_any_key.py [--hidden-desktop] <exe> [<stdin file>]
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

With --hidden-desktop the program runs on a desktop of its own that is never shown (CreateDesktopW;
the name holds this helper's process id, so parallel runs get one each): the window of a screen
program does not appear on the user's desktop and cannot take a key the user types. The screen
program's keys and its state need no window (tools/screen/screen_state.cpp calls libqb directly),
and a run there wrote the same states as on the user's desktop (2026-10-17). The helper holds the
desktop until the program ends; it goes when nothing uses it any more. The console stays this
helper's own, so the key presses still reach END.
"""

from __future__ import annotations

import contextlib
import ctypes
import msvcrt
import os
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


class STARTUPINFOW(ctypes.Structure):
    _fields_ = [
        ("cb", wintypes.DWORD),
        ("lpReserved", wintypes.LPWSTR),
        ("lpDesktop", wintypes.LPWSTR),
        ("lpTitle", wintypes.LPWSTR),
        ("dwX", wintypes.DWORD),
        ("dwY", wintypes.DWORD),
        ("dwXSize", wintypes.DWORD),
        ("dwYSize", wintypes.DWORD),
        ("dwXCountChars", wintypes.DWORD),
        ("dwYCountChars", wintypes.DWORD),
        ("dwFillAttribute", wintypes.DWORD),
        ("dwFlags", wintypes.DWORD),
        ("wShowWindow", wintypes.WORD),
        ("cbReserved2", wintypes.WORD),
        ("lpReserved2", ctypes.c_void_p),
        ("hStdInput", wintypes.HANDLE),
        ("hStdOutput", wintypes.HANDLE),
        ("hStdError", wintypes.HANDLE),
    ]


class PROCESS_INFORMATION(ctypes.Structure):
    _fields_ = [
        ("hProcess", wintypes.HANDLE),
        ("hThread", wintypes.HANDLE),
        ("dwProcessId", wintypes.DWORD),
        ("dwThreadId", wintypes.DWORD),
    ]


GENERIC_ALL = 0x10000000
STARTF_USESTDHANDLES = 0x100
HANDLE_FLAG_INHERIT = 1
STD_OUTPUT_HANDLE = -11
STD_ERROR_HANDLE = -12
WAIT_TIMEOUT = 0x102


def window_station_name(u32) -> str:
    """The name of this process's window station (WinSta0 in an interactive session)."""
    u32.GetProcessWindowStation.restype = wintypes.HANDLE
    u32.GetUserObjectInformationW.argtypes = [
        wintypes.HANDLE,
        ctypes.c_int,
        ctypes.c_void_p,
        wintypes.DWORD,
        ctypes.POINTER(wintypes.DWORD),
    ]
    buf = ctypes.create_unicode_buffer(256)
    needed = wintypes.DWORD()
    UOI_NAME = 2
    if not u32.GetUserObjectInformationW(
        u32.GetProcessWindowStation(), UOI_NAME, buf, ctypes.sizeof(buf), ctypes.byref(needed)
    ):
        raise OSError(f"GetUserObjectInformationW failed (error {ctypes.get_last_error()})")
    return buf.value


class DesktopProcess:
    """The program started on a hidden desktop, with the part of Popen's interface main uses."""

    def __init__(self, k32, exe: str, stdin: int):
        u32 = ctypes.WinDLL("user32", use_last_error=True)
        u32.CreateDesktopW.restype = wintypes.HANDLE
        u32.CreateDesktopW.argtypes = [
            wintypes.LPCWSTR,
            wintypes.LPCWSTR,
            ctypes.c_void_p,
            wintypes.DWORD,
            wintypes.DWORD,
            ctypes.c_void_p,
        ]
        u32.CloseDesktop.argtypes = [wintypes.HANDLE]
        k32.GetStdHandle.restype = wintypes.HANDLE
        k32.GetStdHandle.argtypes = [wintypes.DWORD]
        k32.SetHandleInformation.argtypes = [wintypes.HANDLE, wintypes.DWORD, wintypes.DWORD]
        k32.CreateProcessW.argtypes = [
            wintypes.LPCWSTR,
            wintypes.LPWSTR,
            ctypes.c_void_p,
            ctypes.c_void_p,
            wintypes.BOOL,
            wintypes.DWORD,
            ctypes.c_void_p,
            wintypes.LPCWSTR,
            ctypes.POINTER(STARTUPINFOW),
            ctypes.POINTER(PROCESS_INFORMATION),
        ]
        k32.WaitForSingleObject.argtypes = [wintypes.HANDLE, wintypes.DWORD]
        k32.GetExitCodeProcess.argtypes = [wintypes.HANDLE, ctypes.POINTER(wintypes.DWORD)]
        k32.TerminateProcess.argtypes = [wintypes.HANDLE, wintypes.UINT]
        k32.CloseHandle.argtypes = [wintypes.HANDLE]
        self.k32 = k32
        self.returncode: int | None = None

        name = f"qb64rust-test-{os.getpid()}"
        desktop = u32.CreateDesktopW(name, None, None, 0, GENERIC_ALL, None)
        if not desktop:
            raise OSError(f"CreateDesktopW failed (error {ctypes.get_last_error()})")
        try:
            si = STARTUPINFOW()
            si.cb = ctypes.sizeof(si)
            si.lpDesktop = f"{window_station_name(u32)}\\{name}"
            si.dwFlags = STARTF_USESTDHANDLES
            si.hStdInput = stdin
            si.hStdOutput = k32.GetStdHandle(STD_OUTPUT_HANDLE)
            si.hStdError = k32.GetStdHandle(STD_ERROR_HANDLE)
            for h in (si.hStdInput, si.hStdOutput, si.hStdError):
                if h:
                    k32.SetHandleInformation(h, HANDLE_FLAG_INHERIT, HANDLE_FLAG_INHERIT)
            pi = PROCESS_INFORMATION()
            exe = os.path.abspath(exe)
            command = ctypes.create_unicode_buffer(subprocess.list2cmdline([exe]))
            if not k32.CreateProcessW(
                exe, command, None, None, True, 0, None, None, ctypes.byref(si), ctypes.byref(pi)
            ):
                raise OSError(f"CreateProcessW failed (error {ctypes.get_last_error()})")
        except OSError:
            u32.CloseDesktop(desktop)
            raise
        # Held until the program ends: closed at once, the desktop goes before the program's
        # user32 attaches to it, which then fails to start (0xC0000142, STATUS_DLL_INIT_FAILED).
        self.u32, self.desktop = u32, desktop
        k32.CloseHandle(pi.hThread)
        self.process = pi.hProcess

    def poll(self) -> int | None:
        ended = self.k32.WaitForSingleObject(self.process, 0) != WAIT_TIMEOUT
        if self.returncode is None and ended:
            code = wintypes.DWORD()
            self.k32.GetExitCodeProcess(self.process, ctypes.byref(code))
            self.returncode = code.value
            self.u32.CloseDesktop(self.desktop)
        return self.returncode

    def kill(self) -> None:
        self.k32.TerminateProcess(self.process, 1)


def main() -> int:
    args = sys.argv[1:]
    hidden = bool(args) and args[0] == "--hidden-desktop"
    if hidden:
        args = args[1:]
    if len(args) not in (1, 2):
        print("usage: press_any_key.py [--hidden-desktop] <exe> [<stdin file>]", file=sys.stderr)
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
    # process's own, i.e. the result file. DesktopProcess passes the same three handles.
    with contextlib.ExitStack() as stack:
        if len(args) == 2:
            stdin_file = stack.enter_context(open(args[1], "rb"))
            stdin_fd = stdin_file.fileno()
        else:
            stdin_fd = msvcrt.open_osfhandle(conin, 0)
        try:
            if hidden:
                p = DesktopProcess(k32, args[0], msvcrt.get_osfhandle(stdin_fd))
            else:
                p = subprocess.Popen([args[0]], stdin=stdin_fd)
        except OSError as e:
            print(f"press_any_key: {e}", file=sys.stderr)
            return 125
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
    return exit_status(p.returncode)


def exit_status(code: int) -> int:
    """The value for sys.exit that makes this process end with the code `code`, as Windows reports it
    (unsigned 32 bits). sys.exit takes a C long, 32 bits on Windows: a code from 2**31 up is an
    OverflowError there, and the process ends with 4294967295 whatever the code was. Its negative
    value ends with the code itself (END -2 of a program: 4294967294)."""
    return code - 2**32 if code >= 2**31 else code


if __name__ == "__main__":
    sys.exit(main())
