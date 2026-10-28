$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): a fixed-length string variable passed to a STRING parameter whose
' VARPTR is taken.
DIM f AS STRING * 3
f = "abc"
t f
SYSTEM

SUB t (s$)
    PRINT VARPTR(s$); s$
END SUB
