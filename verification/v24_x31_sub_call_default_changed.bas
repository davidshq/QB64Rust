$CONSOLE:ONLY
' Verification (m2-defxxx-common-arrays, D1): a SUB with a parameter typed by a default, called with a
' variable typed by another default.
DEFINT A-Z
v = 5
bump v
PRINT v
DEFSNG A-Z
bump v
PRINT v; v%
SYSTEM
DEFINT A-Z
SUB bump (n)
    n = n + 1
END SUB
