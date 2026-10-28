$CONSOLE:ONLY
' Verification (qbasic-testcases, review of the change): VARPTR of a whole TYPE parameter (of a member: v61_p1;
' of an element of a member array: v61_p9).
TYPE rec
    a AS INTEGER
    b AS INTEGER
END TYPE
DIM r AS rec
s r
SYSTEM

SUB s (p AS rec)
    PRINT VARPTR(p)
END SUB
