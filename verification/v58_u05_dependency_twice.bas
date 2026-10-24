$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a library named by the program and by the top file of another library it uses (c uses a).
$USELIBRARY:'test/a'
$USELIBRARY:'test/c'
PRINT "main "; top$
ASub
CSub
SYSTEM
