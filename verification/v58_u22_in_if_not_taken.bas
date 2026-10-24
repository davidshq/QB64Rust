$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a missing library named in a $IF branch that is not taken.
$IF 0 THEN
$USELIBRARY:'test/none'
$END IF
PRINT "main "; top$
SYSTEM
