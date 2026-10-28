' TEST: cpp
$CONSOLE:ONLY
' VARSEG and VARPTR of places in a TYPE parameter, as the old compiler writes them (its C++ of
' verification\v61_p1, p9): VARSEG is libqb's check of the place, which may be an element of a member array;
' VARPTR is the place's offset from dblock, as for a variable, even when the argument lies in an array.
TYPE rec
    a AS INTEGER
    m(5) AS INTEGER
END TYPE
DIM r AS rec
s r
SYSTEM

SUB s (p AS rec)
    PRINT VARSEG(p.m(2)); VARSEG(p.a); VARSEG(p)
    PRINT VARPTR(p.m(2)); VARPTR(p.a)
END SUB
