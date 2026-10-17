$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-measured-copies, before its design): a whole-array assignment to or from a _DYNAMIC member
' array that REDIM gave another number of dimensions than it is declared with. The old compiler passes the
' member through its descriptor guard for the declared number (qb64pe.bas UDTDynArrayDescGuard$: error 9 and a
' stand-in descriptor when slot 3 differs) and compares the bound slots of the declared number.
' 1. member (two dimensions now, one declared) to an array of one dimension
' 2. an array of one dimension to that member
' 3. two members, both REDIMmed to the same two dimensions
' 4. member of the declared one dimension to a member of two
' 5. two members of two dimensions declared, both now one of the same bounds
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
TYPE u
    w(1, 1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
DIM x AS t, y AS t, z AS u, z2 AS u
DIM a(1) AS LONG
a(0) = 1: a(1) = 2
REDIM x.v(0 TO 1, 0 TO 0) AS LONG
x.v(0, 0) = 5: x.v(1, 0) = 6
PRINT "1.";: a() = x.v(): PRINT a(0); a(1)
PRINT "2.";: x.v() = a(): PRINT x.v(0, 0); x.v(1, 0)
REDIM y.v(0 TO 1, 0 TO 0) AS LONG
y.v(0, 0) = 7: y.v(1, 0) = 8
PRINT "3.";: x.v() = y.v(): PRINT x.v(0, 0); x.v(1, 0)
REDIM y.v(0 TO 1) AS LONG
y.v(0) = 3: y.v(1) = 4
PRINT "4.";: x.v() = y.v(): PRINT x.v(0, 0); x.v(1, 0); y.v(0); y.v(1)
REDIM z.w(0 TO 3) AS LONG
z.w(0) = 9: z.w(3) = 12
REDIM z2.w(0 TO 3) AS LONG
z2.w(0) = 30: z2.w(3) = 33
PRINT "5.";: z.w() = z2.w(): PRINT z.w(0); z.w(3); z2.w(0)
PRINT "ok"
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
