$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): name(3) = 1 where a SUB of that name is defined below.
s(3) = 1
PRINT s(3)
SYSTEM
SUB s (n)
    PRINT n
END SUB
