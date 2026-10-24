$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): an error handler in the after-main file.
$USELIBRARY:'test/three'
ON ERROR GOTO three_err
PRINT "main "; top$
ERROR 5
PRINT "after"
SYSTEM
