$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): the metacommand after a colon (rejected, as every metacommand is).
PRINT "a": $USELIBRARY:'test/a'
PRINT "main "; top$
SYSTEM
