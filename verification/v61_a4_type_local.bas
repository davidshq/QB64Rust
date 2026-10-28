$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR and VARSEG of the members of a SUB's TYPE variable, VARSEG of
' the whole variable, and of a member of an element of its array of the TYPE; each starts at zero on each call.
TYPE pt
    x AS INTEGER
    y AS LONG
    z AS STRING * 4
END TYPE
s
s
SYSTEM

SUB s
    DIM p AS pt, r(2) AS pt
    PRINT p.x; p.y; ASC(p.z, 1); r(1).y
    PRINT VARPTR(p.x); VARPTR(p.y); VARPTR(p.z); VARSEG(p)
    PRINT VARPTR(r(1).y); VARSEG(r(1).y)
    p.x = 5: p.y = 6: p.z = "abcd": r(1).y = 7
    DEF SEG = VARSEG(p)
    PRINT PEEK(VARPTR(p.x)); PEEK(VARPTR(p.y)); PEEK(VARPTR(p.z))
    POKE VARPTR(p.y), 66
    POKE VARPTR(r(1).y), 77
    DEF SEG
    PRINT p.y; r(1).y
END SUB
