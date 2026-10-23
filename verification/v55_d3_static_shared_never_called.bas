$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): STATIC a(5) in a SUB that is never called, beside a static
' shared main-module array of that name and type.
ON ERROR GOTO h
PRINT "main starts"
DIM SHARED a(3) AS LONG
a(1) = 7
PRINT "main:"; a(1); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC a(5) AS LONG
    PRINT "SUB:"; a(1); UBOUND(a)
END SUB
