$CONSOLE:ONLY
DIM a(1 TO 10) AS INTEGER, c(0 TO 4) AS LONG
PRINT "Offsets of a(1), a(2), a(10):"; VARPTR(a(1)); VARPTR(a(2)); VARPTR(a(10))
PRINT "Offset of c(3):"; VARPTR(c(3))
END
