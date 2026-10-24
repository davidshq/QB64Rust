$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): one library named on two lines (accepted? included twice?).
$USELIBRARY:'test/a'
PRINT "main "; top$
$USELIBRARY:'test/a'
ASub
SYSTEM
