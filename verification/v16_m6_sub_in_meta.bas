$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6, review): a SUB inside an active $IF branch.
s
SYSTEM
$IF WIN THEN
SUB s
    PRINT "s in $IF"
END SUB
$END IF
