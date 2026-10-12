$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 6): DIM of an array that STATIC c(3) declared above it
' in the SUB, with the same bounds.
ON ERROR GOTO h
s: s
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT

SUB s
    STATIC c(3) AS LONG
    DIM c(3) AS LONG
    c(1) = c(1) + 1
    PRINT "SUB:"; c(1); UBOUND(c)
END SUB
