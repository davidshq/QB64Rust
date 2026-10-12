$CONSOLE:ONLY
' Verification (step 11, task 1.1): a tab before $LET.
	$LET FOO = 1
$IF FOO = 1 THEN
    PRINT "foo 1"
$ELSE
    PRINT "foo not 1"
$END IF
SYSTEM
