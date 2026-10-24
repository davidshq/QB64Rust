$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a descriptor without the [LIBRARY INCLUDES] section (nothing included?).
$USELIBRARY:'test/nosection'
PRINT "main "; top$
SYSTEM
