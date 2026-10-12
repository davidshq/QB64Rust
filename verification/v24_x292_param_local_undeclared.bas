$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): in a SUB, an array never declared passed to an array parameter.
ON ERROR GOTO h
outer
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB outer
    fill loc()
    PRINT UBOUND(loc)
END SUB
SUB fill (a())
    a(1) = 5
    PRINT a(1);
END SUB
