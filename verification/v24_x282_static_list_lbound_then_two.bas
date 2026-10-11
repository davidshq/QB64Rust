$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, 6.1): STATIC y(), then LBOUND(y), then y(1, 2): whether LBOUND gave
' the listed array one dimension.
t
SYSTEM
SUB t
    STATIC y()
    PRINT LBOUND(y)
    PRINT y(1, 2)
END SUB
