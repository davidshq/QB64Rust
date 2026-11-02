' _SCREENCLICK on the program's own window, read back by _MOUSEINPUT and _MOUSEBUTTON (tests/sandbox/README.md).
' libqb scales the desktop's place to 0..65535 with the desktop's size, so the place read back may be a pixel off
' by the sandbox's window size, and at a display scale of 125% it was 2 pixels off across (QB64PE-DIVERGENCES.md
' D-056): printed as within 2 pixels.
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
OPEN "out.txt" FOR OUTPUT AS #1
DO WHILE _MOUSEINPUT: LOOP

_SCREENCLICK _SCREENX + 100, _SCREENY + 80
t# = TIMER
DO
    DO WHILE _MOUSEINPUT
        IF _MOUSEBUTTON(1) AND pressed = 0 THEN pressed = -1: mx = _MOUSEX: my = _MOUSEY
        IF pressed AND NOT _MOUSEBUTTON(1) THEN released = -1
    LOOP
    _LIMIT 100
LOOP UNTIL released OR TIMER - t# > 5
PRINT #1, "left button down:"; pressed; "and up:"; released
PRINT #1, "down at 100, 80 within 2 pixels:"; ABS(mx - 100) <= 2 AND ABS(my - 80) <= 2
CLOSE #1
SYSTEM
