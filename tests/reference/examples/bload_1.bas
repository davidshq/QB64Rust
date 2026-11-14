$CONSOLE:ONLY
ON ERROR GOTO trap
DIM a(1 TO 5) AS INTEGER, b(1 TO 5) AS INTEGER
FOR i = 1 TO 5: a(i) = i * 11: NEXT
DEF SEG = VARSEG(a(1))
BSAVE "bload_1.tmp", VARPTR(a(1)), 10
DEF SEG = VARSEG(b(1))
BLOAD "bload_1.tmp", VARPTR(b(1))
DEF SEG
FOR i = 1 TO 5: PRINT b(i);: NEXT: PRINT
KILL "bload_1.tmp"
BLOAD "no such file.tmp", VARPTR(b(1))
END
trap:
PRINT "error"; ERR
RESUME NEXT
