$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Slice (m3-typefields task 3.4): _DYNAMIC member arrays of a variable and of array elements (verification\v35_a,
' v35_g, v35_x58). The first bounds and values, REDIM (plain, _PRESERVE, _RETAIN, another number of dimensions),
' ERASE and every use after it, LBOUND and UBOUND, a whole-value copy and its isolation, the index errors, and an
' index outside on the way to the member. Where the new compiler differs on purpose (QB64PE-DIVERGENCES.md D-004 widened:
' a store through a parent index outside its bounds stores nothing; the old compiler stores into the member of
' element 0) the line was recorded from a program with ERROR 9 in place of that store.
TYPE t
    a AS INTEGER
    v(2) _DYNAMIC AS LONG
    b AS INTEGER
END TYPE
TYPE u
    v(1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
DIM x AS t, y AS t
DIM i AS LONG
PRINT "1. first bounds:"; LBOUND(x.v); UBOUND(x.v); "values:"; x.v(0); x.v(1); x.v(2)
x.a = 1: x.b = 2
FOR i = 0 TO 2: x.v(i) = 10 + i: NEXT
REDIM x.v(1 TO 5) AS LONG
PRINT "2. plain REDIM 1 TO 5:"; LBOUND(x.v); UBOUND(x.v); "values:";
FOR i = 1 TO 5: PRINT x.v(i);: NEXT: PRINT
PRINT "3. a, b:"; x.a; x.b
FOR i = 1 TO 5: x.v(i) = 20 + i: NEXT
REDIM _PRESERVE x.v(1 TO 7) AS LONG
PRINT "4. _PRESERVE 1 TO 7:";
FOR i = 1 TO 7: PRINT x.v(i);: NEXT: PRINT
REDIM _PRESERVE x.v(3 TO 5) AS LONG
PRINT "5. _PRESERVE 3 TO 5:";
FOR i = 3 TO 5: PRINT x.v(i);: NEXT: PRINT
REDIM _RETAIN x.v(4 TO 8) AS LONG
PRINT "6. _RETAIN 4 TO 8:";
FOR i = 4 TO 8: PRINT x.v(i);: NEXT: PRINT
y = x
y.v(5) = 999
PRINT "7. copy, then y.v(5) = 999:"; x.v(5); y.v(5); LBOUND(y.v); UBOUND(y.v)
REDIM x.v(1 TO 2, 3 TO 4) AS LONG
PRINT "8. two dimensions:"; LBOUND(x.v, 1); UBOUND(x.v, 1); LBOUND(x.v, 2); UBOUND(x.v, 2)
x.v(2, 4) = 7: PRINT "   x.v(2, 4) ="; x.v(2, 4)
PRINT "9. one index after two dimensions:"; x.v(1)
ERASE x.v
PRINT "10. after ERASE: UBOUND ="; UBOUND(x.v)
PRINT "11. after ERASE: x.v(1) ="; x.v(1)
PRINT "12. after ERASE: x.v(0) = 1:";: x.v(0) = 1: PRINT
REDIM x.v(0 TO 1) AS LONG
x.v(1) = 5
PRINT "13. REDIM after ERASE:"; LBOUND(x.v); UBOUND(x.v); x.v(1)
PRINT "14. out of range: x.v(2) ="; x.v(2)
PRINT "15. x.v(-1) = 3:";: x.v(-1) = 3: PRINT
PRINT "16. UBOUND(x.v, 2):"; UBOUND(x.v, 2)
PRINT "17. a, b after the REDIMs:"; x.a; x.b; "y.v(4):"; y.v(4)
REDIM p(2) AS u
PRINT "18.";: REDIM p(9).v(4 TO 6) AS LONG: PRINT UBOUND(p(0).v); UBOUND(p(1).v)
PRINT "19.";: PRINT UBOUND(p(9).v)
PRINT "20.";: p(9).v(5) = 55: PRINT p(0).v(5)
PRINT "21.";: ERASE p(9).v: PRINT UBOUND(p(0).v)
PRINT "22.";: PRINT LBOUND(p(1).v)
REDIM q(1) AS u
ERASE q
PRINT "23. REDIM q(1).v after ERASE q:";: REDIM q(1).v(0 TO 3) AS LONG: PRINT
PRINT "done"
SYSTEM
h:
PRINT "[error"; ERR; "]";
RESUME NEXT
