$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a SUB's scalar parameter passed on to a parameter whose address is
' taken, the main module's argument a value.
a 5
SYSTEM

SUB a (x AS INTEGER)
    t x
END SUB

SUB t (p AS INTEGER)
    PRINT VARPTR(p)
END SUB
