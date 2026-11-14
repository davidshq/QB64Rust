' _ALLOWFULLSCREEN read back by Alt+Enter: the program clicks its own window, presses Alt+Enter with Windows'
' keybd_event and reads the functions _FULLSCREEN and _SMOOTH (tests/sandbox/README.md).
DECLARE DYNAMIC LIBRARY "user32"
    SUB keybd_event (BYVAL vk AS _UNSIGNED _BYTE, BYVAL scan AS _UNSIGNED _BYTE, BYVAL flags AS _UNSIGNED LONG, BYVAL extra AS _OFFSET)
END DECLARE
SCREEN _NEWIMAGE(320, 200, 32)
_DELAY 1
_SCREENCLICK _SCREENX + 50, _SCREENY + 50
_DELAY 0.5
OPEN "out.txt" FOR OUTPUT AS #1
PRINT #1, "at the start: "; mode$

altenter
PRINT #1, "after Alt+Enter: "; mode$
altenter
PRINT #1, "after Alt+Enter again: "; mode$
_FULLSCREEN _OFF
_DELAY 1

_ALLOWFULLSCREEN _SQUAREPIXELS
altenter
PRINT #1, "with _ALLOWFULLSCREEN _SQUAREPIXELS, after Alt+Enter: "; mode$
_FULLSCREEN _OFF
_DELAY 1

_ALLOWFULLSCREEN _OFF
altenter
PRINT #1, "with _ALLOWFULLSCREEN _OFF, after Alt+Enter: "; mode$
CLOSE #1
SYSTEM

SUB altenter
    keybd_event &H12, 0, 0, 0
    keybd_event &H0D, 0, 0, 0
    keybd_event &H0D, 0, 2, 0
    keybd_event &H12, 0, 2, 0
    _DELAY 1
END SUB

FUNCTION mode$
    mode$ = "_FULLSCREEN" + STR$(_FULLSCREEN) + " _SMOOTH" + STR$(_SMOOTH)
END FUNCTION
