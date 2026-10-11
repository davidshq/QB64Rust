$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): STATIC a() AS DOUBLE, then the name used with an index with no
' DIM or REDIM: the listed array (LEN 8) or one used without DIM (LEN 4); STATIC y(), then LBOUND(y); STATIC
' z() AS LONG, then z(1, 2) and REDIM z(2, 3) AS LONG below it.
ON ERROR GOTO h
s: s
t
u: u

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC a() AS DOUBLE
    PRINT "LEN(a(1)):";
    PRINT LEN(a(1));
    a(1) = a(1) + 1
    PRINT a(1)
END SUB
SUB t
    STATIC y()
    PRINT "LBOUND(y):";
    PRINT LBOUND(y)
END SUB
SUB u
    STATIC z() AS LONG
    PRINT "z(1, 2):";
    PRINT z(1, 2)
    REDIM _PRESERVE z(2, 3) AS LONG
    z(1, 2) = z(1, 2) + 1
    PRINT z(1, 2)
END SUB
