$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): the lock keys as values in a program with no window (libqb answers 0 there: no
' keyboard of a window to read), and as comparisons.
c = _CAPSLOCK: n = _NUMLOCK: s = _SCROLLLOCK
PRINT c; n; s
PRINT c = 0 OR c = -1; n = 0 OR n = -1; s = 0 OR s = -1
x&& = _CAPSLOCK: PRINT x&&
SYSTEM
