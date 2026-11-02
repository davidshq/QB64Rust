' _SCREENPRINT types into the window in front, the program's own: read back by INKEY$ (tests/sandbox/README.md).
' Only the session's first window gets the keys though each reports the focus, so the program clicks its own window
' first (OpenSpec change desktop-sandbox, design.md "Context").
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
OPEN "out.txt" FOR OUTPUT AS #1
_SCREENCLICK _SCREENX + 100, _SCREENY + 100
_DELAY 0.3
DO WHILE _MOUSEINPUT: LOOP

_SCREENPRINT "abc" + CHR$(13)
t# = TIMER
DO
    k$ = INKEY$
    IF k$ <> "" THEN s$ = s$ + k$
    _LIMIT 100
LOOP UNTIL k$ = CHR$(13) OR TIMER - t# > 5
PRINT #1, "keys read:"; LEN(s$);
FOR i = 1 TO LEN(s$)
    PRINT #1, ASC(s$, i);
NEXT
PRINT #1, ""
CLOSE #1
SYSTEM
