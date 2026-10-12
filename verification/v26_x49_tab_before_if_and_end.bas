$CONSOLE:ONLY
' Verification (step 11, task 1.1): a tab before $IF and before its $END IF (v26_x40 stops at its $END IF).
	$IF 1 THEN
PRINT "in the branch"
	$END IF
PRINT "ok"
SYSTEM
