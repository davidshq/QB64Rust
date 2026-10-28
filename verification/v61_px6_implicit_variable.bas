$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a variable used without DIM, of the parameter's type, passed to a
' parameter whose address is taken.
z! = 5
t z!
PRINT z!
SYSTEM

SUB t (p AS SINGLE)
    PRINT VARPTR(p)
END SUB
