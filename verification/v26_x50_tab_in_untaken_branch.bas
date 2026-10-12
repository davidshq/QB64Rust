$CONSOLE:ONLY
' Verification (step 11, task 1.1): a tab before $ELSE and $END IF inside a branch that is not taken. Do they
' end the branch?
$IF 0 THEN
	$ELSE
PRINT "after the tabbed $ELSE"
	$END IF
PRINT "after the tabbed $END IF"
$END IF
PRINT "ok"
SYSTEM
