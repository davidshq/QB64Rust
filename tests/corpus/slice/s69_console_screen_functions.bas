$CONSOLE:ONLY
' Slice (m3-window-output, task 3.3): the screen and keyboard functions in a program with a console and no window.
' With standard input and output redirected INKEY$ is empty, POS(0) is 1 wherever the output stands and CSRLIN
' does not move as lines are printed (verification\v27_a_console_functions), so only values that are the same on
' every run are printed: never CSRLIN itself.
DIM k AS STRING, r AS LONG
k = INKEY$
PRINT "inkey:"; LEN(k); LEN(INKEY$ + "x")
r = CSRLIN
PRINT "pos:"; POS(0); POS(1) * 2; " "; HEX$(POS(0))
PRINT "abc"; POS(0)
PRINT "csrlin:"; CSRLIN - r; CSRLIN = r
SYSTEM
