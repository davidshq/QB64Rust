' _SCREENICON, _SCREENHIDE and _SCREENSHOW read back by Windows' IsIconic and IsWindowVisible on the program's
' window, and by the functions _SCREENICON and _SCREENEXISTS (tests/sandbox/README.md). Each line is written as it
' is measured, so that a run ended at the time limit shows how far it got.
DECLARE DYNAMIC LIBRARY "user32"
    FUNCTION IsIconic& (BYVAL hwnd AS _OFFSET)
    FUNCTION IsWindowVisible& (BYVAL hwnd AS _OFFSET)
END DECLARE
DIM SHARED h AS _OFFSET
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
h = _WINDOWHANDLE
OPEN "out.txt" FOR OUTPUT AS #1: CLOSE #1
say "at the start, minimized:" + STR$(IsIconic(h) <> 0) + " visible:" + STR$(IsWindowVisible(h) <> 0) + " _SCREENICON:" + STR$(_SCREENICON)

_SCREENICON
_DELAY 1
say "after _SCREENICON, minimized:" + STR$(IsIconic(h) <> 0) + " _SCREENICON:" + STR$(_SCREENICON)

_SCREENSHOW
_DELAY 1
say "after _SCREENSHOW, minimized:" + STR$(IsIconic(h) <> 0) + " visible:" + STR$(IsWindowVisible(h) <> 0)

_SCREENHIDE
_DELAY 1
say "after _SCREENHIDE, visible:" + STR$(IsWindowVisible(h) <> 0) + " _SCREENEXISTS:" + STR$(_SCREENEXISTS)

_SCREENSHOW
_DELAY 1
say "after _SCREENSHOW, visible:" + STR$(IsWindowVisible(h) <> 0)
say "ending"
SYSTEM

SUB say (t$)
    OPEN "out.txt" FOR APPEND AS #1
    PRINT #1, t$
    CLOSE #1
END SUB
