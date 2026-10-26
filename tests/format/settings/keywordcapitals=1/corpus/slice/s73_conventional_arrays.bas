$CONSOLE:ONLY
' Slice (m3-get-put-bload, task 4.3; from verification\v28_e_cmem_arrays): arrays in conventional memory used
' as any array is: ERASE, an
' array listed in COMMON (dynamic to the old compiler), REDIM, a dynamic array, passed to a SUB, LBOUND and
' UBOUND, and the order of the segments.
COMMON SHARED cm() AS INTEGER
DIM SHARED cm(1 TO 20) AS INTEGER
DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER, n AS INTEGER, i AS INTEGER
n = 8
DIM dy(1 TO n) AS LONG
REDIM rd(1 TO 4) AS INTEGER
ON ERROR GOTO handler
' The static ones lie first, in the order of their DIM; the dynamic ones follow from the other end.
PRINT VARSEG(b(1)) - VARSEG(a(1)); VARSEG(cm(1)) > VARSEG(b(1)); VARSEG(dy(1)) > VARSEG(b(1)); VARSEG(rd(1)) > VARSEG(b(1))
PRINT VARSEG(cm(1)) - VARSEG(dy(1)); VARSEG(dy(1)) - VARSEG(rd(1))
PRINT VARPTR(cm(3)); VARPTR(dy(3)); VARPTR(rd(3))
' Values through DEF SEG.
FOR i = 1 TO 10: a(i) = i: NEXT
cm(1) = 258: dy(1) = 65537: rd(2) = 7
DEF SEG = VARSEG(cm(1)): PRINT PEEK(VARPTR(cm(1))); PEEK(VARPTR(cm(1)) + 1)
DEF SEG = VARSEG(dy(1)): PRINT PEEK(VARPTR(dy(1))); PEEK(VARPTR(dy(1)) + 2)
DEF SEG = VARSEG(rd(1)): POKE VARPTR(rd(2)), 9: PRINT rd(2)
DEF SEG
' ERASE: a static one is cleared, a dynamic one released; then allocated again.
ERASE a
PRINT a(3); LBOUND(a); UBOUND(a)
ERASE cm
PRINT "cm erased"
i = cm(1)
REDIM cm(1 TO 5) AS INTEGER
cm(5) = 55: PRINT cm(5); UBOUND(cm)
REDIM _PRESERVE rd(1 TO 8) AS INTEGER
PRINT rd(2); UBOUND(rd)
DEF SEG = VARSEG(rd(1)): PRINT PEEK(VARPTR(rd(2))): DEF SEG
ERASE dy
REDIM dy(1 TO 3) AS LONG
dy(3) = 3: PRINT dy(3)
' Passed to a SUB.
CALL sb(a(), rd())
PRINT a(2); rd(1); UBOUND(rd)
DEF SEG = VARSEG(rd(1)): PRINT PEEK(VARPTR(rd(1))): DEF SEG
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT

SUB sb (p() AS INTEGER, r() AS INTEGER)
    p(2) = 22
    ERASE p
    p(2) = 23
    REDIM r(1 TO 12) AS INTEGER
    r(1) = 77
END SUB
