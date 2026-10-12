$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): $DYNAMIC inside a SUB and for the SUB below it; STATIC a(3) and
' a DIM in a SUB ... STATIC below $DYNAMIC.
ON ERROR GOTO h
s1: s1
s2: s2
s3: s3
DIM m(5) AS LONG
PRINT "main DIM m(5), below the calls and above the SUBs in the file, then REDIM m(8):";
REDIM m(8) AS LONG
PRINT UBOUND(m)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s1
    '$DYNAMIC
    STATIC a(3) AS LONG
    a(1) = a(1) + 1
    PRINT "s1, STATIC a(3) below $DYNAMIC: a(1)"; a(1); "then REDIM a(5): UBOUND";
    REDIM a(5) AS LONG
    PRINT UBOUND(a)
END SUB
SUB s2 STATIC
    DIM b(3) AS LONG
    b(1) = b(1) + 1
    PRINT "s2 STATIC, DIM b(3) below s1's $DYNAMIC: b(1)"; b(1); "then REDIM b(5): UBOUND";
    REDIM b(5) AS LONG
    PRINT UBOUND(b)
END SUB
'$STATIC
SUB s3 STATIC
    DIM c(3) AS LONG
    c(1) = c(1) + 1
    PRINT "s3 STATIC, DIM c(3) below $STATIC: c(1)"; c(1); "then REDIM c(5): UBOUND";
    REDIM c(5) AS LONG
    PRINT UBOUND(c)
END SUB
