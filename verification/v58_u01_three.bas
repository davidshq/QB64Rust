$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): a library with a file of each kind: a CONST of the top file, a GOSUB routine of the after-main file, a SUB of the bottom file.
$USELIBRARY:'test/three'
PRINT "main "; top$; THREE_K
GOSUB three_handler
ThreeSub
SYSTEM
