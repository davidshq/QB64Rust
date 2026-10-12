$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array used without DIM in the main module: bounds per number
' of indexes, a suffix, a first use that is a read, LBOUND first, a first use skipped by GOTO, REDIM of one,
' another type beside a declared array.
ON ERROR GOTO h
z(10) = 4
PRINT "1. z(10) = 4: z(10), LBOUND, UBOUND"; z(10); LBOUND(z); UBOUND(z)
PRINT "2. z(11) = 1:";
z(11) = 1
PRINT
m(2, 3) = 5
PRINT "3. two indexes: UBOUND 1, UBOUND 2, LBOUND 2, m(2, 3), m(10, 10)"; UBOUND(m, 1); UBOUND(m, 2); LBOUND(m, 2); m(2, 3); m(10, 10)
t(1, 2, 3) = 1
PRINT "4. three indexes: UBOUND 3"; UBOUND(t, 3)
s$(2) = "ab": i%(3) = 2.5: d#(1) = 1# / 3: q&&(10) = 5000000000
PRINT "5. suffixes: "; s$(2); UBOUND(s$); i%(3); d#(1); q&&(10)
PRINT "6. the first use is a read:"; r(4); UBOUND(r)
PRINT "7. LBOUND(lb) first, then lb(10) = 3:"; LBOUND(lb); UBOUND(lb);
lb(10) = 3: PRINT lb(10);
PRINT " UBOUND(lb, 2):";
PRINT UBOUND(lb, 2)
PRINT "8. the first use skipped by GOTO:";
GOTO sk
w(1) = 1
sk:
PRINT UBOUND(w); w(1)
PRINT "9. REDIM z(20):";
REDIM z(20)
PRINT UBOUND(z); z(10)
DIM c(3) AS LONG
c!(7) = 1.5
PRINT "10. c!(7) beside DIM c(3) AS LONG: c!(7), UBOUND(c), UBOUND(c!)"; c!(7); UBOUND(c); UBOUND(c!)
a = 5: a(2) = 3
PRINT "11. a scalar and an array of one name: a, a(2)"; a; a(2)
PRINT "12. an index in a float and in a string of digits:"; z(9.6);
z(VAL("3")) = 8: PRINT z(3)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
