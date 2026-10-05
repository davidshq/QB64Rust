$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): $IF after a statement and a colon.
PRINT "a": $IF WIN THEN
PRINT "b"
$END IF
SYSTEM
