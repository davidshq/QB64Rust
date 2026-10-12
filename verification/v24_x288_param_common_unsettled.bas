$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): a COMMON array passed before anything gave it bounds; the SUB gives it two dimensions.
ON ERROR GOTO h
COMMON c() AS LONG
make c()
PRINT UBOUND(c, 1); UBOUND(c, 2); c(1, 2)
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB make (a() AS LONG)
    REDIM a(3, 4) AS LONG
    a(1, 2) = 7
END SUB
