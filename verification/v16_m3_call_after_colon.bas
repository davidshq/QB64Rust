$CONSOLE:ONLY
' Verification (m2-parser-breadth, M3, group 6): a SUB called between two colons.
PRINT "a": s: PRINT "b"
SUB s
    PRINT "in s"
END SUB
SYSTEM
