' Screen (m4-builtins-remaining, task 3.2): _BLINK OFF with a line in a blinking colour, _BLINK as a function, the
' display statements in each form and the display functions in a text window (verification\v58_s05, s06, f14): the
' values that are the machine's (the refresh rate, the lock keys) as comparisons.
SCREEN 0
_BLINK OFF
COLOR 20, 1: PRINT "x"
COLOR 7, 0
PRINT _BLINK
_BLINK ON: PRINT _BLINK
_BLINK OFF
_GLRENDER _BEHIND: _GLRENDER _ONTOP: _GLRENDER _BEHIND
_DISPLAYORDER _SOFTWARE, _HARDWARE: _DISPLAYORDER _SOFTWARE
_ALLOWFULLSCREEN _STRETCH, _SMOOTH: _ALLOWFULLSCREEN _OFF: _ALLOWFULLSCREEN
_FPS 30: _FPS _AUTO
PRINT _SCALEDWIDTH; _SCALEDHEIGHT; _SMOOTH; _DESKTOPREFRESHRATE > 0; _DISPLAY; _DISPLAY = _DEST
c = _CAPSLOCK: n = _NUMLOCK: s = _SCROLLLOCK
PRINT c = 0 OR c = -1; n = 0 OR n = -1; s = 0 OR s = -1
SYSTEM
