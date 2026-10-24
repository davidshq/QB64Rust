$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): no $ERRORLOCATION and the same recursion.
PRINT "start"
deep 1
PRINT "not reached"
SYSTEM

SUB deep (n AS LONG)
    deep n + 1
END SUB
