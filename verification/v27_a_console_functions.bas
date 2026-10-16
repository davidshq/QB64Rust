$CONSOLE:ONLY
' Verification (m3-window-output, task 1.3): INKEY$, CSRLIN and POS in a program with a console and no window.
DIM k AS STRING, r AS LONG
k = INKEY$
PRINT LEN(k)
r = CSRLIN
PRINT r - CSRLIN
PRINT POS(0)
PRINT "abc"; POS(0)
PRINT "abc"; : PRINT POS(0); CSRLIN - r
PRINT HEX$(POS(0)); POS(0) / 4; CSRLIN - CSRLIN
SYSTEM
