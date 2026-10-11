$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, task 4.2): READ into elements of a _BIT array, static and dynamic: values
' inside the element's range, then one outside it (9 into a _BIT * 4).
ON ERROR GOTO h
DIM b(3) AS _BIT * 4, u(3) AS _UNSIGNED _BIT * 3
n = 3
DIM d(n) AS _BIT * 4
READ b(1), b(2), u(1), d(3)
PRINT b(0); b(1); b(2); b(3); u(0); u(1); u(2); d(2); d(3)
PRINT "9 into a _BIT * 4:";
READ b(3)
PRINT b(3)
DATA 5, -8, 7, -3, 9
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
