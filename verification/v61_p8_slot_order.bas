$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): where an argument's slot lies among the variables in conventional
' memory declared before and after its call, in the main module and in a SUB.
DIM a AS INTEGER
PRINT VARPTR(a)
t 3
DIM b AS INTEGER
PRINT VARPTR(b)
s
SYSTEM

SUB s
    DIM c AS INTEGER
    PRINT VARPTR(c)
    t 4
    DIM d AS INTEGER
    PRINT VARPTR(d)
END SUB

SUB t (p AS INTEGER)
    PRINT VARPTR(p)
END SUB
