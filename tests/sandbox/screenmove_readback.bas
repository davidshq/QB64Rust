' _SCREENMOVE x, y and _SCREENMOVE _MIDDLE, read back by _SCREENX and _SCREENY (tests/sandbox/README.md).
' The window's place before a move and the desktop's size follow the sandbox's window: not printed.
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
OPEN "out.txt" FOR OUTPUT AS #1

_SCREENMOVE 100, 120
_DELAY 0.5
PRINT #1, "after _SCREENMOVE 100, 120:"; _SCREENX; _SCREENY

' libqb centers the window's inside in the monitor's work area: across, on the desktop's middle; down, above it
' by half the taskbar at most.
_SCREENMOVE _MIDDLE
_DELAY 0.5
across = _SCREENX + _WIDTH / 2 - _DESKTOPWIDTH / 2
down = _SCREENY + _HEIGHT / 2 - _DESKTOPHEIGHT / 2
PRINT #1, "after _SCREENMOVE _MIDDLE, centered across:"; ABS(across) <= 1
PRINT #1, "centered down, within 40 pixels above:"; down <= 1 AND down >= -40
PRINT #1, "moved:"; _SCREENX <> 100 OR _SCREENY <> 120
CLOSE #1
SYSTEM
