$CONSOLE:ONLY
' Slice (m3-get-put-bload, task 4.3): DEF SEG, VARSEG, VARPTR, PEEK, POKE, BSAVE and BLOAD on arrays of the main
' module (verification\v28_b_bload, v28_d_varptr_index). An array named in VARPTR or VARSEG lies in
' conventional memory: the arrays one after the other in the order of their DIM, each rounded up to 16 bytes.
' Only differences of segments are printed, never a segment itself.
DIM SHARED calls AS INTEGER
DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER, c(0 TO 4) AS LONG, m(1 TO 3, 1 TO 4) AS INTEGER
DIM i AS INTEGER, s AS SINGLE, l AS LONG, q AS _INTEGER64
DIM SHARED g(1 TO 10) AS INTEGER
ON ERROR GOTO handler
' Offsets: the index from the lower bound times the element's size.
PRINT VARPTR(a(1)); VARPTR(a(2)); VARPTR(a(10)); VARPTR(c(0)); VARPTR(c(3)); VARPTR(m(1, 1)); VARPTR(m(2, 1)); VARPTR(m(1, 2))
PRINT VARSEG(b(1)) - VARSEG(a(1)); VARSEG(c(0)) - VARSEG(b(1)); VARSEG(m(1, 1)) - VARSEG(c(0)); VARSEG(g(1)) - VARSEG(a(1))
PRINT VARSEG(a(1)) = VARSEG(a(7)); VARSEG(a(1)) > 0
' An offset is an _INTEGER64 value; the index of VARPTR is evaluated and checked, that of VARSEG is not.
PRINT STR$(VARPTR(a(2))); VARPTR(a(2)) + 1; VARPTR(a(3)) / 4; VARPTR(a(10)) * 100000; -VARPTR(a(2))
i = VARPTR(a(3)): s = VARPTR(a(4)): q = VARPTR(a(5))
PRINT i; s; q
l = VARPTR(a(f%(2)))
PRINT "VARPTR:"; l; "calls"; calls
calls = 0
l = VARSEG(a(f%(3))) - VARSEG(a(1))
PRINT "VARSEG:"; l; "calls"; calls
l = VARPTR(a(11))
PRINT "after 11:"; l
l = VARSEG(a(99)) - VARSEG(a(1))
PRINT "segment of 99:"; l
' POKE and PEEK through DEF SEG.
DEF SEG = VARSEG(a(1))
POKE VARPTR(a(1)), 65
POKE VARPTR(a(1)) + 1, 1
POKE VARPTR(a(3)), 255
POKE VARPTR(a(3)) + 1, 255
PRINT a(1); a(2); a(3)
a(4) = &H1234
PRINT PEEK(VARPTR(a(4))); PEEK(VARPTR(a(4)) + 1)
POKE VARPTR(a(5)), 300: PRINT a(5)
POKE VARPTR(a(5)), -1: PRINT a(5)
s = VARPTR(a(6)): POKE s, 7.6: PRINT a(6); PEEK(s)
DEF SEG
' BSAVE and BLOAD.
FOR i = 1 TO 10: a(i) = i * 11: NEXT
DEF SEG = VARSEG(a(1))
BSAVE "s72_bload.tmp", VARPTR(a(1)), 20
DEF SEG
OPEN "s72_bload.tmp" FOR BINARY AS #1
PRINT "file length"; LOF(1)
CLOSE #1
DEF SEG = VARSEG(b(1))
BLOAD "s72_bload.tmp", VARPTR(b(1))
DEF SEG
FOR i = 1 TO 10: PRINT b(i);: NEXT: PRINT
' With no offset: the one of the BSAVE, in the current segment.
FOR i = 1 TO 10: a(i) = 0: NEXT
DEF SEG = VARSEG(a(1))
BLOAD "s72_bload.tmp"
DEF SEG
FOR i = 1 TO 10: PRINT a(i);: NEXT: PRINT
' In a procedure, on a shared array.
CALL fill
FOR i = 1 TO 10: PRINT g(i);: NEXT: PRINT
' A file that does not exist.
BLOAD "s72_missing.tmp", VARPTR(b(1))
PRINT "after the missing file"
KILL "s72_bload.tmp"
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT

SUB fill
    DEF SEG = VARSEG(g(1))
    BLOAD "s72_bload.tmp", VARPTR(g(1))
    POKE VARPTR(g(10)), 9
    DEF SEG
END SUB

FUNCTION f% (n AS INTEGER)
    calls = calls + 1
    f% = n
END FUNCTION
