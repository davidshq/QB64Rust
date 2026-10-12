$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 9): a whole member array as an array argument: of a variable,
' of an array element, with two dimensions, of STRING; what ERASE, REDIM and the bounds do in the procedure,
' and an index outside in the base.
TYPE t1
    id AS LONG
    v(2 TO 5) AS LONG
    g(1 TO 2, 0 TO 2) AS INTEGER
    s(0 TO 1) AS STRING
    tail AS LONG
END TYPE
ON ERROR GOTO h
DIM t AS t1, a(2) AS t1
t.id = 1: t.tail = 9
fill t.v()
PRINT "1. fill t.v():"; t.v(2); t.v(5); t.id; t.tail
fill a(1).v()
PRINT "2. fill a(1).v():"; a(1).v(5); a(0).v(5); a(2).v(5)
PRINT "3. two dimensions, the bounds in the SUB:";
grid t.g()
PRINT ":"; t.g(2, 2)
t.s(1) = "x"
names t.s()
PRINT "4. a STRING member array: ["; t.s(0); "]["; t.s(1); "]"
PRINT "5. ERASE in the SUB:";
wipe t.v()
PRINT ":"; t.v(2); t.v(5); t.id; t.tail; UBOUND(t.v)
PRINT "6. REDIM in the SUB:";
t.v(3) = 33
again t.v()
PRINT ":"; t.v(3); UBOUND(t.v)
PRINT "7. fill a(9).v():";
a(0).v(5) = 0
fill a(9).v()
PRINT ":"; a(0).v(5)
PRINT "8. fill a(one(1)).v():";
fill a(one(1)).v()
PRINT
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB fill (x() AS LONG)
    DIM i AS LONG
    FOR i = LBOUND(x) TO UBOUND(x): x(i) = i * 2: NEXT
END SUB

SUB grid (x() AS INTEGER)
    PRINT LBOUND(x, 1); UBOUND(x, 1); LBOUND(x, 2); UBOUND(x, 2);
    x(2, 2) = 22
END SUB

SUB names (x() AS STRING)
    x(0) = "zero": x(1) = x(1) + "!"
END SUB

SUB wipe (x() AS LONG)
    ERASE x
END SUB

SUB again (x() AS LONG)
    REDIM x(2 TO 5) AS LONG
    PRINT " after REDIM";
    REDIM x(0 TO 9) AS LONG
    PRINT " after the second";
END SUB

FUNCTION one (n AS LONG)
    PRINT n;
    one = 1
END FUNCTION
