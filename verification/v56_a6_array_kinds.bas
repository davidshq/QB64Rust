$CONSOLE:ONLY
' Verification (m4-memory-records, task 1.1): VARPTR and VARSEG of a whole array (no index) and of elements of
' arrays of strings, fixed-length strings and a TYPE, of two dimensions and dynamic; ERASE and REDIM of them.
TYPE rec
    x AS INTEGER
    y AS LONG
END TYPE
DIM a AS INTEGER
DIM s2(1 TO 2, 1 TO 3) AS STRING
DIM f2(1 TO 2, 1 TO 3) AS STRING * 2
DIM r2(1 TO 2, 1 TO 3) AS rec
REDIM rd(5) AS rec
REDIM fd(5) AS STRING * 3
PRINT "a"; VARPTR(a)
PRINT "s2"; VARPTR(s2(1, 1)); VARPTR(s2(2, 1)); VARPTR(s2(1, 3)); VARPTR(s2(2, 3)); VARSEG(s2(1, 1))
PRINT "f2"; VARPTR(f2(1, 1)); VARPTR(f2(2, 1)); VARPTR(f2(1, 3)); VARPTR(f2(2, 3))
PRINT "r2"; VARPTR(r2(1, 1)); VARPTR(r2(2, 1)); VARPTR(r2(1, 3).y); VARPTR(r2(2, 3).y)
PRINT "rd"; VARPTR(rd(0)); VARPTR(rd(4).y); VARSEG(rd(5)) - VARSEG(rd(0))
PRINT "fd"; VARPTR(fd(0)); VARPTR(fd(4)); VARSEG(fd(5)) - VARSEG(fd(0))
PRINT "whole"; VARPTR(s2); VARPTR(f2); VARPTR(r2); VARPTR(rd); VARSEG(s2)
PRINT VARSEG(f2) - VARSEG(f2(1, 1)); VARSEG(r2) - VARSEG(r2(1, 1)); VARSEG(rd) - VARSEG(rd(0))
s2(2, 3) = "text": f2(2, 3) = "ab": r2(2, 3).y = 515: rd(4).y = 260: fd(4) = "xyz"
DEF SEG = VARSEG(s2(1, 1)): PRINT PEEK(VARPTR(s2(2, 3)))
DEF SEG = VARSEG(f2(1, 1)): PRINT PEEK(VARPTR(f2(2, 3)) + 1)
DEF SEG = VARSEG(r2(1, 1)): PRINT PEEK(VARPTR(r2(2, 3).y) + 1)
DEF SEG = VARSEG(rd(0)): PRINT PEEK(VARPTR(rd(4).y) + 1)
DEF SEG = VARSEG(fd(0)): PRINT PEEK(VARPTR(fd(4)) + 2)
DEF SEG
REDIM rd(9) AS rec
PRINT "rd again"; VARPTR(rd(9).y)
ERASE s2
PRINT "s2 erased"; VARPTR(s2(2, 3)); LEN(s2(2, 3))
ON ERROR GOTO h
PRINT VARPTR(s2(3, 1))
PRINT VARPTR(r2(1, 4).x)
SYSTEM
h:
PRINT "error"; ERR
RESUME NEXT
