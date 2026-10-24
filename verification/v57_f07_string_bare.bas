$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a program FUNCTION h$ as a statement: h$ 1.
h$ 1
PRINT "ok"
SYSTEM
FUNCTION h$ (a)
    PRINT "in h"
    h$ = "x"
END FUNCTION
