$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): STATIC a(), then the array used with two indexes after REDIM
' a(2, 3).
ON ERROR GOTO h
s: s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC a() AS LONG
    REDIM _PRESERVE a(2, 3) AS LONG
    a(2, 3) = a(2, 3) + 1
    PRINT a(2, 3);
END SUB
