$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a SUB's local passed to a parameter whose address is taken.
w
SYSTEM

SUB w
    DIM q AS INTEGER
    q = 5
    t q
    PRINT q
END SUB

SUB t (p AS INTEGER)
    PRINT VARPTR(p)
END SUB
