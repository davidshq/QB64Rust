$CONSOLE:ONLY
' Verification (step 11, task 1.1): $IF after a statement and a colon.
x = 1: $IF 1 THEN
PRINT "in the branch"
$END IF
PRINT "ok"
SYSTEM
