$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a DEFxxx between two SUBs, and main-module statements below a
' SUB that has a DEFxxx inside.
SUB s1
    DEFINT A-Z
    v = 2.5: PRINT "s1:"; LEN(v)
END SUB
w = 2.5: PRINT "main below s1:"; LEN(w)
DEFDBL A-Z
s1: s2
SYSTEM
SUB s2
    v = 2.5: PRINT "s2:"; LEN(v)
END SUB
