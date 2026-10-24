$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): an unknown metacommand in a $IF branch that is not taken.
$IF 0 THEN
$NOSUCHTHING
$END IF
PRINT "ok"
SYSTEM
