$CONSOLE:ONLY
' Verification (fixes-switch, review): study 39 row 14, RUN in a FUNCTION called in an expression, with a second
' FUNCTION after it in the same expression. The count is kept in the process's environment, which RUN does not
' clear.
c = VAL(ENVIRON$("V62_G4_COUNT")) + 1
ENVIRON "V62_G4_COUNT=" + LTRIM$(STR$(c))
PRINT "run"; c
IF c > 1 THEN PRINT "restarted": SYSTEM
PRINT f(1) + g(2)
PRINT "after"
SYSTEM

FUNCTION f& (x AS LONG)
    RUN
END FUNCTION

FUNCTION g& (x AS LONG)
    PRINT "g ran"
    g = x
END FUNCTION
