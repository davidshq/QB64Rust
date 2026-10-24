$CONSOLE:ONLY
' Verification (m4-builtins-remaining, task 2.1): $ERRORLOCATION:OFF and the same recursion: the message ends with "Enable
' $ErrorLocation:ON for source location details.".
$ERRORLOCATION:OFF
PRINT "start"
deep 1
PRINT "not reached"
SYSTEM

SUB deep (n AS LONG)
    deep n + 1
END SUB
