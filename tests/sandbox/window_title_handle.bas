' _TITLE and _WINDOWHANDLE read back: the title by Windows' own text of the window, the handle by Windows'
' IsWindow, and _WINDOWHASFOCUS after the program clicks its window (tests/sandbox/README.md). _TITLE$ is not
' read: QB64pe's build gave bytes that are not the title (tests/sandbox/README.md, "Not read back").
DECLARE DYNAMIC LIBRARY "user32"
    FUNCTION IsWindow& (BYVAL hwnd AS _OFFSET)
    FUNCTION GetWindowTextA& (BYVAL hwnd AS _OFFSET, text AS STRING, BYVAL size AS LONG)
END DECLARE
DIM h AS _OFFSET
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
OPEN "out.txt" FOR OUTPUT AS #1
h = _WINDOWHANDLE
PRINT #1, "_WINDOWHANDLE is a window:"; IsWindow(h) <> 0

_TITLE "QB64Rust sandbox title"
_DELAY 0.5
t$ = SPACE$(80)
n = GetWindowTextA(h, t$, 80)
PRINT #1, "the window's text: ["; LEFT$(t$, n); "]"

_SCREENCLICK _SCREENX + 50, _SCREENY + 50
_DELAY 0.5
PRINT #1, "_WINDOWHASFOCUS after a click on it:"; _WINDOWHASFOCUS
CLOSE #1
SYSTEM
