$CONSOLE
$SCREENHIDE
' Verification (m4-builtins-remaining, task 2.1): in a program with a (hidden) window: the function _DISPLAY in screen 0 and after
' SCREEN _NEWIMAGE beside _DEST, _SCALEDWIDTH, _SCALEDHEIGHT, _SMOOTH, _DESKTOPREFRESHRATE as a comparison (it is
' the machine's), _BLINK, and the lock keys as comparisons (the machine's state).
DIM v(12) AS LONG
v(0) = _DISPLAY
SCREEN _NEWIMAGE(320, 200, 32)
v(1) = _DISPLAY: v(2) = _DEST: v(3) = _SCALEDWIDTH: v(4) = _SCALEDHEIGHT: v(5) = _SMOOTH
v(6) = _DESKTOPREFRESHRATE: v(7) = _BLINK
c = _CAPSLOCK: n = _NUMLOCK: s = _SCROLLLOCK
_DEST _CONSOLE
PRINT v(0); v(1) = v(2); v(1) < 0; v(3); v(4); v(5); v(6) > 0; v(7)
PRINT c = 0 OR c = -1; n = 0 OR n = -1; s = 0 OR s = -1
SYSTEM
