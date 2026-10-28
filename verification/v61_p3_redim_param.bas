$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): an array parameter given new bounds by REDIM in the SUB, then VARSEG
' and VARPTR of its element (as InitImageData of qbasic_testcases), for an INTEGER array and an array of a TYPE
' (its InitDesignData); the caller sees the bytes poked.
TYPE design
    x AS INTEGER
    y AS INTEGER
END TYPE
REDIM img(1) AS INTEGER
REDIM des(1) AS design
grow img()
growd des()
PRINT UBOUND(img); img(1); img(4)
PRINT UBOUND(des); des(0).y; des(2).x
SYSTEM

SUB grow (z() AS INTEGER)
    REDIM z(1 TO 4)
    DEF SEG = VARSEG(z(1))
    PRINT VARPTR(z(1)); VARPTR(z(4))
    POKE VARPTR(z(1)), 7
    POKE VARPTR(z(4)), 8
    DEF SEG
END SUB

SUB growd (t() AS design)
    REDIM t(0 TO 2) AS design
    DEF SEG = VARSEG(t(0))
    PRINT VARPTR(t(0)); VARPTR(t(2).x)
    POKE VARPTR(t(0)) + 2, 5
    POKE VARPTR(t(2).x), 6
    DEF SEG
END SUB
