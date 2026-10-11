$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): SHARED a() in a SUB below a local DIM of that name and type,
' and below a use of the name without DIM.
ON ERROR GOTO h
DIM a(3) AS LONG
DIM b(3)
a(1) = 7: b(1) = 8
s
PRINT "main:"; a(1); b(1)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    PRINT "b(1) before SHARED b():";
    PRINT b(1)
    SHARED b()
    PRINT "b(1) after it:";
    PRINT b(1)
    b(1) = 9
    DIM a(5) AS LONG
    a(1) = 1
    SHARED a() AS LONG
    PRINT "a(1), UBOUND(a) after DIM a(5), then SHARED a():";
    PRINT a(1); UBOUND(a)
END SUB
