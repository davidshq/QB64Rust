$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a library whose top file uses another library (c uses a): the dependency's files come first.
$USELIBRARY:'test/c'
PRINT "main "; top$
CSub
SYSTEM
