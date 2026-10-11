$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM: as the first declaration, new bounds and new elements,
' the type kept without AS, a growing bound in a loop, of a static array, of an array made by DIM with a
' variable bound, bounds that give no element, a bound that raises, strings, several arrays in one statement,
' a suffix.
ON ERROR GOTO h
DIM n AS LONG
PRINT "1. the first REDIM declares:";
REDIM r(5) AS LONG
r(2) = 7: PRINT LBOUND(r); UBOUND(r); r(2)
PRINT "2. new bounds, new elements:";
REDIM r(1 TO 20) AS LONG
PRINT LBOUND(r); UBOUND(r); r(2)
PRINT "3. the type kept without AS:";
REDIM x(5) AS LONG
REDIM x(10)
x(1) = 100000: PRINT x(1); UBOUND(x); LEN(x(1))
PRINT "4. one REDIM in a loop, bound 1, 2, 3:";
FOR n = 1 TO 3
    REDIM g(n) AS LONG
    g(n) = n
NEXT
PRINT UBOUND(g); g(3); g(2)
PRINT "5. REDIM of a static array:";
DIM s(5) AS LONG: s(1) = 3
REDIM s(9) AS LONG
PRINT UBOUND(s); s(1)
PRINT "6. REDIM of an array made by DIM d(n):";
n = 4
DIM d(n) AS LONG
d(1) = 5
REDIM d(8) AS LONG
PRINT UBOUND(d); d(1)
PRINT "7. bounds that give no element, REDIM r(n) with n = -1:";
r(2) = 6
n = -1
REDIM r(n) AS LONG
PRINT " then LBOUND(r), UBOUND(r):";
PRINT LBOUND(r); UBOUND(r);
PRINT " r(2):";
PRINT r(2)
PRINT "8. a bound that raises, REDIM y(ASC of empty TO 5) of an array 1 TO 3:";
REDIM y(1 TO 3) AS LONG: y(1) = 11
REDIM y(ASC("") TO 5) AS LONG
PRINT LBOUND(y); UBOUND(y); y(1)
PRINT "9. STRING and STRING * 3 elements are new:";
REDIM t(2) AS STRING: t(1) = "abc": REDIM t(3) AS STRING: PRINT "["; t(1); "]";
REDIM f(2) AS STRING * 3: f(1) = "abc": REDIM f(3) AS STRING * 3: PRINT ASC(f(1)); LEN(f(1))
PRINT "10. two arrays in one statement:";
REDIM m1(2) AS LONG, m2(3) AS INTEGER
PRINT UBOUND(m1); UBOUND(m2); LEN(m1(0)); LEN(m2(0))
PRINT "11. a suffix, REDIM q%(4) then REDIM q%(6):";
REDIM q%(4): q%(1) = 2.5
REDIM q%(6)
PRINT q%(1); UBOUND(q%)
PRINT "12. two dimensions, REDIM w(2, 3) then REDIM w(1 TO 2, 5):";
REDIM w(2, 3) AS LONG: w(2, 3) = 4
REDIM w(1 TO 2, 5) AS LONG
PRINT LBOUND(w, 1); UBOUND(w, 1); UBOUND(w, 2); w(2, 3)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
