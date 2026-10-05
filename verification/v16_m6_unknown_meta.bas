$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): an unknown metacommand inside an active and an inactive branch.
$IF 0 THEN
$NOSUCHTHING
$END IF
PRINT "inactive unknown ok"
$NOSUCHTHING
SYSTEM
