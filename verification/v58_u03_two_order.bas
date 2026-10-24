$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): two libraries, a then b: the order of their top files and bottom files.
$USELIBRARY:'test/a'
$USELIBRARY:'test/b'
PRINT "main "; top$
ASub
BSub
SYSTEM
