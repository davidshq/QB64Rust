$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): a program FUNCTION f as a statement: CALL f(1).
CALL f(1)
PRINT "ok"
SYSTEM
FUNCTION f (a)
    PRINT "in f"
    f = a
END FUNCTION
