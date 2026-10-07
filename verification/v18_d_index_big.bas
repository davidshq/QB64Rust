$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): indexes far out of range (beyond 32 and 64 bits).
DIM x(10) AS LONG
ON ERROR GOTO h
x(0) = 5
PRINT "x(2147483648):"; x(2147483648)
PRINT "x(4294967296):"; x(4294967296)
PRINT "x(-4294967296):"; x(-4294967296)
PRINT "x(1E+30):"; x(1E+30)
big&& = 9223372036854775807
PRINT "x(big&&):"; x(big&&)
SYSTEM

h:
PRINT "handler"; ERR
RESUME NEXT
