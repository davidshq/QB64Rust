$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): COMMON inside a SUB.
s
SYSTEM
SUB s
    COMMON x
    PRINT x
END SUB
