$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR of a SUB's variable used without DIM, as usersubs of
' qbasic_testcases does with HighFore, and of an implicit INTEGER read before it is named.
s 3
s 4
SYSTEM

SUB s (fore)
    HighFore = 0
    DEF SEG = VARSEG(HighFore)
    POKE VARPTR(HighFore) + 1, fore
    DEF SEG
    PRINT VARPTR(HighFore); HighFore; z%
    PRINT VARPTR(z%)
END SUB
