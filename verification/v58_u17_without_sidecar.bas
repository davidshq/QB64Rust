$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a library in a program without a .root sidecar: the release's own folder has no libraries (rejected?).
$USELIBRARY:'test/a'
PRINT "main "; top$
SYSTEM
