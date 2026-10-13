$CONSOLE:ONLY
' Verification (step 11, task 2.1): $ELSE and $END IF after a statement and a colon, inside a branch that is not
' taken (as v26_x50 for a tab). Do they end the branch?
$IF 0 THEN
x = 1: $ELSE
PRINT "after the $ELSE behind a colon"
x = 1: $END IF
PRINT "after the $END IF behind a colon"
$END IF
PRINT "ok"
SYSTEM
