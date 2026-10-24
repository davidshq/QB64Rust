$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a descriptor whose IncAtTop has no value: only the bottom file is included.
$USELIBRARY:'test/empty'
PRINT "main "; top$
EmptyBottom
SYSTEM
