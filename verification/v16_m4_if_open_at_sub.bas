$CONSOLE:ONLY
' Verification (m2-parser-breadth, M4, group 6): an IF still open at a SUB header.
IF 1 THEN
    PRINT 1
SUB s
END SUB
SYSTEM
