$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): STATIC y(), then LBOUND(y), then DIM y(2, 5): whether LBOUND gave the listed array one dimension for a DIM.
ON ERROR GOTO h
t
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB t
    STATIC y()
    PRINT LBOUND(y);
    DIM y(2, 5)
    PRINT UBOUND(y, 2)
END SUB
