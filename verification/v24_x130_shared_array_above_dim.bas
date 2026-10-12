$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): SHARED a() in a SUB that stands above the main module's DIM in
' the file.
SUB s
    SHARED a() AS LONG
    PRINT a(2); UBOUND(a)
END SUB
ON ERROR GOTO h
DIM a(3) AS LONG
a(2) = 8
s

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
