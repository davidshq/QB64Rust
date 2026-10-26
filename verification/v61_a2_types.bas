$CONSOLE:ONLY
' Verification (qbasic-testcases, task 1.1): VARPTR and VARSEG of a SUB's locals of every numeric type, in the
' order of their DIM, with locals no VARPTR names between them; PEEK and POKE through them.
s
s
SYSTEM

SUB s
    DIM b AS _BYTE, i AS INTEGER, l AS LONG, q AS _INTEGER64
    DIM n1 AS LONG, n2 AS DOUBLE
    DIM sg AS SINGLE, d AS DOUBLE, f AS _FLOAT, o AS _OFFSET
    DIM ub AS _UNSIGNED _BYTE, ui AS _UNSIGNED INTEGER, ul AS _UNSIGNED LONG
    DIM n3 AS INTEGER
    DIM uq AS _UNSIGNED _INTEGER64, uo AS _UNSIGNED _OFFSET
    PRINT "b"; VARPTR(b); VARSEG(b)
    PRINT "i"; VARPTR(i); VARSEG(i)
    PRINT "l"; VARPTR(l); VARSEG(l)
    PRINT "q"; VARPTR(q); VARSEG(q)
    PRINT "sg"; VARPTR(sg); VARSEG(sg)
    PRINT "d"; VARPTR(d); VARSEG(d)
    PRINT "f"; VARPTR(f); VARSEG(f)
    PRINT "o"; VARPTR(o); VARSEG(o)
    PRINT "ub"; VARPTR(ub); VARSEG(ub)
    PRINT "ui"; VARPTR(ui); VARSEG(ui)
    PRINT "ul"; VARPTR(ul); VARSEG(ul)
    PRINT "uq"; VARPTR(uq); VARSEG(uq)
    PRINT "uo"; VARPTR(uo); VARSEG(uo)
    PRINT b; i; l; q; d
    n1 = 1: n2 = 2: n3 = 3
    i = 258: l = -2: d = 1.5
    DEF SEG = VARSEG(i)
    PRINT PEEK(VARPTR(i)); PEEK(VARPTR(i) + 1)
    POKE VARPTR(i), 3
    PRINT i
    PRINT PEEK(VARPTR(l)); PEEK(VARPTR(l) + 3)
    POKE VARPTR(l) + 3, 0
    PRINT l
    PRINT PEEK(VARPTR(d) + 6); PEEK(VARPTR(d) + 7)
    POKE VARPTR(b), 200
    PRINT b
    DEF SEG
    PRINT n1; n2; n3
END SUB
