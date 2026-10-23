$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): STATIC a(5) in a SUB beside a shared main-module array that
' REDIM SHARED declares: the bounds the main module and the SUB see.
ON ERROR GOTO h
REDIM SHARED a(3) AS LONG
PRINT "main, after the REDIM:"; UBOUND(a)
s
PRINT "main:"; a(1); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC a(5) AS LONG
    a(1) = a(1) + 1
    PRINT "SUB:"; a(1); UBOUND(a)
END SUB
