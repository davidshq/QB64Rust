$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): DEF SEG, VARSEG, VARPTR, PEEK, POKE, BSAVE and BLOAD on arrays
' of the main module, in a program with a console.
DIM a(1 TO 10) AS INTEGER, b(1 TO 10) AS INTEGER, c(0 TO 4) AS LONG, m(1 TO 3, 1 TO 4) AS INTEGER
DIM i AS INTEGER, n AS INTEGER
DIM SHARED g(1 TO 6) AS INTEGER
' Offsets: the index from the lower bound times the element's size.
PRINT VARPTR(a(1)); VARPTR(a(2)); VARPTR(a(10)); VARPTR(c(0)); VARPTR(c(3)); VARPTR(m(1, 1)); VARPTR(m(2, 1)); VARPTR(m(1, 2))
' Segments: the arrays lie one after the other, each rounded up to 16 bytes.
PRINT VARSEG(b(1)) - VARSEG(a(1)); VARSEG(c(0)) - VARSEG(b(1)); VARSEG(m(1, 1)) - VARSEG(c(0)); VARSEG(g(1)) - VARSEG(a(1))
PRINT VARSEG(a(1)) = VARSEG(a(7)); VARSEG(a(1)) > 0
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
DEF SEG
' BSAVE and BLOAD.
FOR i = 1 TO 10: a(i) = i * 11: NEXT
DEF SEG = VARSEG(a(1))
BSAVE "v28_b_bload.tmp", VARPTR(a(1)), 20
DEF SEG
OPEN "v28_b_bload.tmp" FOR BINARY AS #1
PRINT "file length"; LOF(1)
CLOSE #1
DEF SEG = VARSEG(b(1))
BLOAD "v28_b_bload.tmp", VARPTR(b(1))
DEF SEG
FOR i = 1 TO 10: PRINT b(i);: NEXT: PRINT
' BLOAD at another offset, and with no offset (the one of the BSAVE, in the current segment).
DEF SEG = VARSEG(g(1))
BLOAD "v28_b_bload.tmp", VARPTR(g(3))
DEF SEG
FOR i = 1 TO 6: PRINT g(i);: NEXT: PRINT
FOR i = 1 TO 10: a(i) = 0: NEXT
DEF SEG = VARSEG(a(1))
BLOAD "v28_b_bload.tmp"
DEF SEG
FOR i = 1 TO 10: PRINT a(i);: NEXT: PRINT
' In a procedure, on a shared array.
CALL fill
FOR i = 1 TO 6: PRINT g(i);: NEXT: PRINT
' A file that does not exist.
ON ERROR GOTO handler
BLOAD "v28_b_missing.tmp", VARPTR(b(1))
PRINT "after the missing file"
KILL "v28_b_bload.tmp"
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT

SUB fill
    DEF SEG = VARSEG(g(1))
    POKE VARPTR(g(6)), 9
    BLOAD "v28_b_bload.tmp", VARPTR(g(1))
    DEF SEG
END SUB
