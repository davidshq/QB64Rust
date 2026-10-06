$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): the same constant name in main and in a SUB.
CONST k = 1
s
PRINT "main:"; k
SYSTEM
SUB s
    CONST k = 2
    PRINT "in s:"; k
END SUB
