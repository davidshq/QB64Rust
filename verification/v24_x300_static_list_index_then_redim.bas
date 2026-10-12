$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, group 7): STATIC y(), then y(1) = 5, then REDIM y(2, 5).
ON ERROR GOTO h
t
SYSTEM
h:
PRINT " [error"; ERR; "]";
RESUME NEXT
SUB t
    STATIC y()
    y(1) = 5
    REDIM y(2, 5)
    PRINT UBOUND(y, 2)
END SUB
