$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (m3-typefields, before its design): the runtime errors of a _DYNAMIC member: an index outside
' its bounds, a bound upper below lower, REDIM of a member of an erased parent, an element of an erased member,
' UBOUND of a dimension the member does not have, an index with the wrong number of dimensions after REDIM.
TYPE t
    v(1) _DYNAMIC AS LONG
END TYPE
ON ERROR GOTO h
DIM x AS t
PRINT "1. x.v(5):"; x.v(5)
PRINT "2. x.v(-1) = 3:";: x.v(-1) = 3: PRINT
PRINT "3. (REDIM x.v(lo TO hi) with 3 TO 1 ends the program: v35_g2)"
PRINT "4. UBOUND(x.v, 2):"; UBOUND(x.v, 2)
REDIM p(1) AS t
ERASE p
PRINT "5. REDIM p(1).v after ERASE p:";: REDIM p(1).v(0 TO 3) AS LONG: PRINT
PRINT "6. p(1).v(0) after ERASE p:"; p(1).v(0)
ERASE x.v
PRINT "7. LBOUND after ERASE x.v:"; LBOUND(x.v)
PRINT "8. x.v(0) = 1 after ERASE x.v:";: x.v(0) = 1: PRINT
REDIM x.v(0 TO 1, 0 TO 1) AS LONG
PRINT "9. x.v(1) after two dimensions:"; x.v(1)
PRINT "10. (a bound too large: v35_g3)"
PRINT "done"
SYSTEM
h:
PRINT "[error"; ERR; "at"; _ERRORLINE; "]";
RESUME NEXT
