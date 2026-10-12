$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, review of group 6): in a SUB ... STATIC, DIM of a name that is a CONST
' of the main module.
CONST k = 1
s
SYSTEM

SUB s STATIC
    DIM k
    PRINT k
END SUB
