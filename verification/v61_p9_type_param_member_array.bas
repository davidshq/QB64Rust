$CONSOLE:ONLY
' Verification (qbasic-testcases, review of the change): VARSEG and VARPTR of an element of a member array of a
' TYPE parameter, called with a TYPE variable and with an element of a TYPE array (outside DBLOCK), beside VARSEG
' of the parameter; a POKE through them reaches the variable, and for the array's element lands elsewhere
' (VARPTR of the whole parameter: v61_px9).
TYPE rec
    a AS INTEGER
    m(5) AS INTEGER
END TYPE
DIM r AS rec
DIM ra(3) AS rec
r.m(2) = 1: ra(1).m(2) = 2
s r
s ra(1)
PRINT r.m(2); ra(1).m(2)
SYSTEM

SUB s (p AS rec)
    PRINT VARSEG(p.m(2)); VARSEG(p); VARPTR(p.m(2)); p.m(2)
    DEF SEG = VARSEG(p.m(2))
    POKE VARPTR(p.m(2)), 42
    DEF SEG
END SUB
