$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a descriptor whose IncAfterMain file is missing (rejected?).
$USELIBRARY:'test/broken'
PRINT "main "; top$
SYSTEM
