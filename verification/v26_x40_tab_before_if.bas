$CONSOLE:ONLY
' Verification (step 11, task 1.1): a tab before $IF (its $END IF has none).
	$IF 1 THEN
PRINT "in the branch"
$END IF
PRINT "ok"
SYSTEM
