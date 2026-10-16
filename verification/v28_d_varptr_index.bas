$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): the index inside VARPTR and VARSEG (is a FUNCTION in it called?),
' their values in STR$, an operation and a store, and an index out of bounds.
DIM SHARED calls AS INTEGER
DIM a(1 TO 10) AS INTEGER, i AS INTEGER, s AS SINGLE, l AS LONG, q AS _INTEGER64
l = VARPTR(a(f%(2)))
PRINT "VARPTR:"; l; "calls"; calls
calls = 0
l = VARSEG(a(f%(3))) - VARSEG(a(1))
PRINT "VARSEG:"; l; "calls"; calls
PRINT STR$(VARPTR(a(2))); VARPTR(a(2)) + 1; VARPTR(a(3)) / 4; VARPTR(a(10)) * 100000
i = VARPTR(a(3)): s = VARPTR(a(4)): q = VARPTR(a(5))
PRINT i; s; q
PRINT VARPTR(a(3)) = 4; VARPTR(a(3)) > VARPTR(a(2)); -VARPTR(a(2))
ON ERROR GOTO handler
l = VARPTR(a(11))
PRINT "after 11:"; l
l = VARPTR(a(0))
PRINT "after 0:"; l
l = VARSEG(a(99)) - VARSEG(a(1))
PRINT "segment of 99:"; l
SYSTEM
handler:
PRINT "error"; ERR
RESUME NEXT

FUNCTION f% (n AS INTEGER)
    calls = calls + 1
    f% = n
END FUNCTION
