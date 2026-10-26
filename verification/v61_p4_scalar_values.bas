$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR and VARSEG of a scalar parameter called with values: a
' literal, a variable in parentheses and an expression; each call site has its own slot, and the variable keeps
' its value.
x% = 5
t 3
t (x%)
t x% + 1
t 3
PRINT x%
SYSTEM

SUB t (p AS INTEGER)
    PRINT VARPTR(p); VARSEG(p); p
    DEF SEG = VARSEG(p)
    POKE VARPTR(p), 99
    DEF SEG
    PRINT p
END SUB
