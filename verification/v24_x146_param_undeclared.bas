$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): an array never declared passed to an array parameter.
ON ERROR GOTO h
fill nx()
PRINT nx(1); UBOUND(nx)

SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB fill (a())
    a(1) = 5
END SUB
