$CONSOLE:ONLY
' Verification (m2-parser-breadth, M6): an inactive $ERROR.
$IF LINUX THEN
$ERROR not on Linux
$END IF
PRINT "ok"
SYSTEM
