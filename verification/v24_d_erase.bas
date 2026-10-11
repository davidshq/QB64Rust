$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): ERASE: static arrays of each element kind and of two dimensions,
' a dynamic array (then its elements, a second ERASE, REDIM, the DIM that made it run again), several names,
' an array beside a scalar of its name.
ON ERROR GOTO h
TYPE rec
    n AS LONG
    s AS STRING * 2
    d AS DOUBLE
END TYPE
DIM i AS LONG, n AS LONG
DIM a(5) AS LONG: a(2) = 9
ERASE a
PRINT "1. static LONG: UBOUND, a(2)"; UBOUND(a); a(2)
DIM t(2) AS STRING, f(2) AS STRING * 3, u(2) AS rec, d(2) AS DOUBLE, m(1 TO 2, 3) AS INTEGER
t(1) = "abc": f(1) = "abc": u(1).n = 5: u(1).s = "xy": u(1).d = 1.5: d(1) = 1.5: m(2, 3) = 7
ERASE t, f, u
ERASE d, m
PRINT "2. static STRING, STRING * 3, a user type, DOUBLE, two dimensions:";
PRINT "["; t(1); "]"; ASC(f(1)); LEN(f(1)); u(1).n; ASC(u(1).s); u(1).d; d(1); m(2, 3); LBOUND(m, 1); UBOUND(m, 2)
REDIM y(5) AS LONG: y(1) = 4
ERASE y
PRINT "3. dynamic: UBOUND(y):";
PRINT UBOUND(y);
PRINT " y(1):";
PRINT y(1);
PRINT " ERASE y again:";
ERASE y
PRINT " REDIM y(2): UBOUND, y(1)";
REDIM y(2) AS LONG
PRINT UBOUND(y); y(1)
PRINT "4. DIM z(n), ERASE z, and the same DIM again, in a loop:";
n = 3
FOR i = 1 TO 2
    DIM z(n + i) AS LONG
    z(1) = z(1) + i
    PRINT UBOUND(z); z(1);
    ERASE z
NEXT
PRINT
REDIM ds(2) AS STRING: ds(1) = "abc"
ERASE ds
PRINT "5. dynamic STRING: UBOUND(ds):";
PRINT UBOUND(ds)
DIM sc(3) AS LONG
sc = 5: sc(1) = 6
ERASE sc
PRINT "6. an array beside a scalar of its name: sc, sc(1)"; sc; sc(1)
im(4) = 3
ERASE im
PRINT "7. an array used without DIM: UBOUND, im(4)"; UBOUND(im); im(4)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
