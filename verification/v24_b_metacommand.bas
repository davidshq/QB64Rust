$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): $DYNAMIC and $STATIC in a comment: a DIM with constant bounds
' below each, an array declared above, a DIM skipped by GOTO, REM, DIM SHARED, an array used without DIM, the
' same DIM run twice.
ON ERROR GOTO h
DIM i AS LONG
DIM s0(5) AS LONG
'$DYNAMIC
DIM a(5) AS LONG
a(1) = 3
PRINT "1. DIM a(5) below $DYNAMIC, then REDIM a(8): UBOUND, a(1)";
REDIM a(8) AS LONG
PRINT UBOUND(a); a(1)
'$STATIC
DIM b(5) AS LONG
PRINT "2. DIM b(5) below $STATIC, then REDIM b(8): UBOUND";
REDIM b(8) AS LONG
PRINT UBOUND(b)
'$DYNAMIC
PRINT "3. REDIM s0(8), s0 declared above the first $DYNAMIC: UBOUND";
REDIM s0(8) AS LONG
PRINT UBOUND(s0)
PRINT "4. DIM c(5) below $DYNAMIC skipped by GOTO, c(1) = 1:";
GOTO skip
DIM c(5) AS LONG
skip:
c(1) = 1
PRINT " UBOUND(c):";
PRINT UBOUND(c)
PRINT "5. an array used without DIM below $DYNAMIC, im(3) = 4: UBOUND, then REDIM im(20): UBOUND";
im(3) = 4
PRINT UBOUND(im);
REDIM im(20)
PRINT UBOUND(im)
PRINT "6. DIM SHARED g(5) below $DYNAMIC, in a SUB, then REDIM g(7):";
DIM SHARED g(5) AS LONG
g(2) = 4
showg
REDIM g(7) AS LONG
PRINT UBOUND(g)
PRINT "7. one DIM l(3) below $DYNAMIC run twice:";
FOR i = 1 TO 2
    DIM l(3) AS LONG
NEXT
PRINT
REM $STATIC
DIM d(5) AS LONG
PRINT "8. DIM d(5) below REM $STATIC, then REDIM d(8): UBOUND";
REDIM d(8) AS LONG
PRINT UBOUND(d)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB showg
    PRINT g(2); UBOUND(g);
END SUB
