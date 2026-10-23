$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): STATIC a(5) in a SUB beside a shared main-module array of
' that name and type that is dynamic ($DYNAMIC): which allocation comes first, and what the main DIM does.
ON ERROR GOTO h
'$DYNAMIC
PRINT "main, before the DIM:"
DIM SHARED a(3) AS LONG
PRINT "main, after the DIM:"; UBOUND(a)
a(1) = 7
s: s
PRINT "main:"; a(1); UBOUND(a)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC a(5) AS LONG
    PRINT "SUB:"; a(1); UBOUND(a)
    a(1) = a(1) + 1
END SUB
