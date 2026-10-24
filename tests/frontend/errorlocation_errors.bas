' TEST: check-fail
' $ERRORLOCATION's errors (m4-builtins-remaining design D6; verification\v58_e08-e18): ON and OFF in one program
' is the old compiler's error at the program's first $ERRORLOCATION line (the last setting is the program's, the
' first line that differs is reported); the forms it does not know (bare, another word, a blank for the colon or
' after it, a comment after the setting: the whole line is compared) are its "Syntax error".
$CONSOLE:ONLY
$ERRORLOCATION:ON
PRINT "a"
$ERRORLOCATION:OFF
$ERRORLOCATION
$ERRORLOCATION:MAYBE
$ERRORLOCATION ON
$ERRORLOCATION: ON
$errorlocation:on ' a comment
PRINT "ok"
