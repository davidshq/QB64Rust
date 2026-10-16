$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): a _DYNAMIC member array of a plain variable: its first
' bounds and values, REDIM of it (plain, _PRESERVE, _RETAIN, another number of dimensions), ERASE, LBOUND and
' UBOUND, a whole-value copy and its isolation.
TYPE t
    a AS INTEGER
    v(2) _DYNAMIC AS LONG
    b AS INTEGER
END TYPE
ON ERROR GOTO h
DIM x AS t, y AS t
DIM i AS LONG
PRINT "1. first bounds:"; LBOUND(x.v); UBOUND(x.v); "values:"; x.v(0); x.v(1); x.v(2)
PRINT "2. (LEN(x) is rejected: v35_x20)"
x.a = 1: x.b = 2
FOR i = 0 TO 2: x.v(i) = 10 + i: NEXT
REDIM x.v(1 TO 5) AS LONG
PRINT "3. plain REDIM 1 TO 5:"; LBOUND(x.v); UBOUND(x.v); "values:";
FOR i = 1 TO 5: PRINT x.v(i);: NEXT: PRINT
PRINT "4. a, b:"; x.a; x.b
FOR i = 1 TO 5: x.v(i) = 20 + i: NEXT
REDIM _PRESERVE x.v(1 TO 7) AS LONG
PRINT "5. _PRESERVE 1 TO 7:";
FOR i = 1 TO 7: PRINT x.v(i);: NEXT: PRINT
REDIM _PRESERVE x.v(3 TO 5) AS LONG
PRINT "6. _PRESERVE 3 TO 5:";
FOR i = 3 TO 5: PRINT x.v(i);: NEXT: PRINT
REDIM _RETAIN x.v(4 TO 8) AS LONG
PRINT "7. _RETAIN 4 TO 8:";
FOR i = 4 TO 8: PRINT x.v(i);: NEXT: PRINT
y = x
y.v(5) = 999
PRINT "8. copy y = x, then y.v(5) = 999: x.v(5) ="; x.v(5); " y.v(5) ="; y.v(5); LBOUND(y.v); UBOUND(y.v)
REDIM x.v(1 TO 2, 3 TO 4) AS LONG
PRINT "9. two dimensions:"; LBOUND(x.v, 1); UBOUND(x.v, 1); LBOUND(x.v, 2); UBOUND(x.v, 2)
x.v(2, 4) = 7: PRINT "   x.v(2, 4) ="; x.v(2, 4)
ERASE x.v
PRINT "10. after ERASE: UBOUND ="; UBOUND(x.v)
PRINT "11. after ERASE: x.v(1) ="; x.v(1)
REDIM x.v(0 TO 1) AS LONG
x.v(1) = 5
PRINT "12. REDIM after ERASE:"; LBOUND(x.v); UBOUND(x.v); x.v(1)
PRINT "13. out of range: x.v(2) ="; x.v(2)
PRINT "14. a, b after the REDIMs:"; x.a; x.b
SYSTEM
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
