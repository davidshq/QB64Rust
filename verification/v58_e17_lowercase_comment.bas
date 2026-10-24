$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $errorlocation:on in lower case with a comment after it (accepted?), and the
' recursion of v58_e01.
$errorlocation:on ' a comment
PRINT "start"
deep 1
SYSTEM

SUB deep (n AS LONG)
    deep n + 1
END SUB
