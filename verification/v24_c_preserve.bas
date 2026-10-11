$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): REDIM _PRESERVE: growing and shrinking, strings, fixed-length
' strings, a user type, two dimensions, another lower bound, an array that is not allocated, a static array,
' bounds that give no element, a bound that raises.
ON ERROR GOTO h
TYPE rec
    n AS LONG
    s AS STRING * 2
END TYPE
DIM i AS LONG, j AS LONG, n AS LONG
REDIM p(3) AS LONG
FOR i = 0 TO 3: p(i) = i + 1: NEXT
REDIM _PRESERVE p(5) AS LONG
PRINT "1. grow 3 to 5: p(3), p(5)"; p(3); p(5)
REDIM _PRESERVE p(1) AS LONG
PRINT "   shrink to 1: UBOUND, p(1)"; UBOUND(p); p(1)
REDIM _PRESERVE p(3) AS LONG
PRINT "   grow to 3 again: p(1), p(2), p(3)"; p(1); p(2); p(3)
REDIM t(1) AS STRING: t(0) = "zero": t(1) = "keep"
REDIM _PRESERVE t(4) AS STRING
PRINT "2. strings: "; t(0); " "; t(1); LEN(t(4))
REDIM _PRESERVE t(0) AS STRING
PRINT "   shrunk to 0: "; t(0); UBOUND(t)
REDIM f(1) AS STRING * 3: f(1) = "abc"
REDIM _PRESERVE f(3) AS STRING * 3
PRINT "3. fixed-length strings: "; f(1); ASC(f(3)); LEN(f(3))
REDIM u(1) AS rec: u(1).n = 7: u(1).s = "xy"
REDIM _PRESERVE u(2) AS rec
PRINT "4. a user type:"; u(1).n; u(1).s; u(2).n; ASC(u(2).s)
REDIM g(1, 1) AS LONG
g(0, 0) = 1: g(1, 0) = 2: g(0, 1) = 3: g(1, 1) = 4
REDIM _PRESERVE g(2, 1) AS LONG
PRINT "5. two dimensions, (1, 1) holding 1 2 3 4 by first index fastest, to (2, 1):"
FOR j = 0 TO 1
    PRINT "   second index"; j; ":";
    FOR i = 0 TO 2: PRINT g(i, j);: NEXT
    PRINT
NEXT
REDIM _PRESERVE g(2, 2) AS LONG
PRINT "   then to (2, 2):"
FOR j = 0 TO 2
    PRINT "   second index"; j; ":";
    FOR i = 0 TO 2: PRINT g(i, j);: NEXT
    PRINT
NEXT
REDIM _PRESERVE g(1, 1) AS LONG
PRINT "   then to (1, 1):"; g(0, 0); g(1, 0); g(0, 1); g(1, 1)
REDIM b(3) AS LONG
FOR i = 0 TO 3: b(i) = i + 1: NEXT
REDIM _PRESERVE b(10 TO 15) AS LONG
PRINT "6. another lower bound, 0 TO 3 holding 1 2 3 4 to 10 TO 15:";
FOR i = 10 TO 15: PRINT b(i);: NEXT
PRINT
REDIM e(3) AS LONG: e(1) = 5
ERASE e
PRINT "7. of an array not allocated:";
REDIM _PRESERVE e(4) AS LONG
PRINT UBOUND(e); e(1)
DIM s(5) AS LONG: s(1) = 3
PRINT "8. of a static array:";
REDIM _PRESERVE s(9) AS LONG
PRINT UBOUND(s); s(1)
PRINT "9. bounds that give no element, n = -1:";
n = -1
REDIM _PRESERVE p(n) AS LONG
PRINT " then LBOUND(p), UBOUND(p):";
PRINT LBOUND(p); UBOUND(p);
PRINT " p(1):";
PRINT p(1)
REDIM y(1 TO 3) AS LONG: y(1) = 11
PRINT "10. a bound that raises, to (ASC of empty TO 5) from 1 TO 3:";
REDIM _PRESERVE y(ASC("") TO 5) AS LONG
PRINT LBOUND(y); UBOUND(y); y(1); y(0)
REDIM k(3)
k(1) = 1.5
REDIM _PRESERVE k(5)
PRINT "11. without AS, a SINGLE array:"; k(1); UBOUND(k)
REDIM _PRESERVE x1(2) AS LONG, x2(2) AS LONG
PRINT "12. two arrays in one statement, both new:"; UBOUND(x1); UBOUND(x2)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
