$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): an array never declared passed to a parameter the SUB indexes with two indexes.
ON ERROR GOTO h
fill nx()
PRINT nx(1, 2); UBOUND(nx, 1); UBOUND(nx, 2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB fill (a())
    a(1, 2) = 5
END SUB
