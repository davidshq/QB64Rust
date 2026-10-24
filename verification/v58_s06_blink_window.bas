$CONSOLE
$SCREENHIDE
' Verification (m4-builtins-remaining, task 2.1): _BLINK as a function and a statement in a program with a hidden text window,
' and a line printed in a blinking colour with blinking off.
SCREEN 0
b0 = _BLINK
_BLINK OFF
b1 = _BLINK
_BLINK ON
b2 = _BLINK
_BLINK OFF
COLOR 20, 1: PRINT "x"
b3 = _BLINK
_DEST _CONSOLE
PRINT b0; b1; b2; b3
SYSTEM
