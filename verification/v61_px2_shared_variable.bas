$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a DIM SHARED variable passed to a parameter whose address is taken.
DIM SHARED g AS INTEGER
g = 5
t g
PRINT g
SYSTEM

SUB t (p AS INTEGER)
    PRINT VARPTR(p)
END SUB
