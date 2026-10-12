$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): DIM with a bound that is not constant, in the main module:
' bounds, the statement run twice, bounds that give no element, float bounds, a bound that raises, first
' values, two dimensions, DIM SHARED.
ON ERROR GOTO h
DIM n AS LONG, m AS LONG, i AS LONG
n = 4
DIM d(n) AS LONG
d(4) = 9: PRINT "1. DIM d(n), n = 4: LBOUND, UBOUND, d(4)"; LBOUND(d); UBOUND(d); d(4)
PRINT "2. one DIM run twice in a loop:";
FOR i = 1 TO 2
    DIM e(n + i) AS LONG
    e(1) = e(1) + 1
NEXT
PRINT " UBOUND(e), e(1)"; UBOUND(e); e(1)
PRINT "3. DIM f(m), m = -1 (no element):";
m = -1
DIM f(m) AS LONG
PRINT " then LBOUND(f), UBOUND(f):";
PRINT LBOUND(f); UBOUND(f)
PRINT "4. then f(0) = 1:";
f(0) = 1
PRINT
PRINT "5. DIM k(5 TO m), m = 4, 5 in a loop:";
FOR m = 4 TO 5
    DIM k(5 TO m) AS LONG
NEXT
PRINT " LBOUND, UBOUND"; LBOUND(k); UBOUND(k)
PRINT "6. float bounds, DIM q(x) with x = 2.5, DIM r(1.5 + z TO 3.5 + z) with z = 0:";
x = 2.5: z = 0
DIM q(x) AS LONG
DIM r(1.5 + z TO 3.5 + z) AS LONG
PRINT LBOUND(q); UBOUND(q); LBOUND(r); UBOUND(r)
PRINT "7. a bound that raises, DIM s(ASC of an empty string):";
DIM s(ASC("")) AS LONG
PRINT " then UBOUND(s):";
PRINT UBOUND(s)
PRINT "8. first values of STRING, STRING * 3, DOUBLE elements:";
DIM t1(n) AS STRING, t2(n) AS STRING * 3, t3(n) AS DOUBLE
PRINT "["; t1(2); "]"; LEN(t1(2)); ASC(t2(2)); LEN(t2(2)); t3(2)
PRINT "9. DIM u(n, 1 TO n):";
DIM u(n, 1 TO n) AS INTEGER
PRINT LBOUND(u, 1); UBOUND(u, 1); LBOUND(u, 2); UBOUND(u, 2);
u(4, 4) = 7: PRINT u(4, 4); u(0, 1)
PRINT "10. an index outside: d(5) = 1, PRINT d(-1):";
d(5) = 1
PRINT d(-1)
PRINT "11. DIM SHARED sh(n), seen in a SUB:";
DIM SHARED sh(n) AS LONG
sh(2) = 6
showsh
PRINT "12. one constant and one variable bound, DIM v(3, n):";
DIM v(3, n) AS LONG
PRINT UBOUND(v, 1); UBOUND(v, 2)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB showsh
    PRINT sh(2); UBOUND(sh)
END SUB
