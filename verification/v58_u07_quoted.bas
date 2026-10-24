$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a descriptor with a comment line, another section before the right one, a key in lower case with blanks, a value in quotes with a comment after it.
$USELIBRARY:'test/quoted'
PRINT "main "; top$
QSub
SYSTEM
