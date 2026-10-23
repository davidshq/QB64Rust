$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): STATIC a(5) AS INTEGER in a SUB beside a shared main-module
' array a(3) AS LONG of another type.
ON ERROR GOTO h
DIM SHARED a(3) AS LONG
a(1) = 7
s: s
PRINT "main:"; a(1); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC a(5) AS INTEGER
    PRINT "SUB:"; a(1); UBOUND(a)
    a(1) = a(1) + 1
END SUB
