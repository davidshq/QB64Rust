$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a main-module INTEGER variable passed to a parameter whose address is
' taken.
DIM x AS INTEGER
x = 5
t x
PRINT x
SYSTEM

SUB t (p AS INTEGER)
    PRINT VARPTR(p)
END SUB
