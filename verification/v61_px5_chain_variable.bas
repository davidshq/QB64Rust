$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a main-module variable passed to a parameter that is passed on to a
' parameter whose address is taken.
y% = 8
a y%
PRINT y%
SYSTEM

SUB a (x AS INTEGER)
    t x
END SUB

SUB t (p AS INTEGER)
    PRINT VARPTR(p)
END SUB
