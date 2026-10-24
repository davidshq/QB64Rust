$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:ON and a SUB of the main file that runs out of stack: the critical
' error's message names the line of the recursive call and "main module".
$ERRORLOCATION:ON
PRINT "start"
deep 1
PRINT "not reached"
SYSTEM

SUB deep (n AS LONG)
    deep n + 1
END SUB
