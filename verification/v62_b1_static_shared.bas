$CONSOLE:ONLY
' Verification (fixes-switch, task 1.2): study 39 row 4, STATIC a(5) in a SUB beside a shared static main-module
' array of that name and type, the SUB called three times (bounds, elements kept, the shared array untouched).
ON ERROR GOTO h
DIM SHARED a(3) AS LONG
a(1) = 7
PRINT "main:"; a(1); LBOUND(a); UBOUND(a)
s: s: s
PRINT "main:"; a(1); LBOUND(a); UBOUND(a)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC a(5) AS LONG
    a(1) = a(1) + 1: a(5) = a(5) + 10
    PRINT "SUB:"; a(1); a(5); LBOUND(a); UBOUND(a)
END SUB
