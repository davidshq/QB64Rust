$CONSOLE:ONLY
' Verification (step 11, task 1.1): $LET after a statement and a colon.
x = 1: $LET FOO = 1
$IF FOO = 1 THEN
    PRINT "foo 1"
$ELSE
    PRINT "foo not 1"
$END IF
SYSTEM
