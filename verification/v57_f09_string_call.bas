$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a program FUNCTION h$ as a statement: CALL h$(1).
CALL h$(1)
PRINT "ok"
SYSTEM
FUNCTION h$ (a)
    PRINT "in h"
    h$ = "x"
END FUNCTION
