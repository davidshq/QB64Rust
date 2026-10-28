$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR of a recursive FUNCTION's local at each depth, before and after
' the deeper call, and of a local of a SUB called after it returns.
PRINT r%(3)
s
SYSTEM

FUNCTION r% (n AS INTEGER)
    DIM z AS INTEGER
    z = n
    PRINT "in"; n; VARPTR(z)
    IF n > 0 THEN r% = r%(n - 1) + z
    PRINT "out"; n; VARPTR(z); z
END FUNCTION

SUB s
    DIM y AS INTEGER
    PRINT VARPTR(y)
END SUB
