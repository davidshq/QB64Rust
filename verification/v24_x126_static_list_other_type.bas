$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): STATIC a() AS DOUBLE then DIM a(3) without a type: listed or not
' (LEN 8 or 4).
ON ERROR GOTO h
s: s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC a() AS DOUBLE
    DIM a(3)
    a(1) = a(1) + 1
    PRINT a(1); LEN(a(1));
END SUB
