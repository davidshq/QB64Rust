$CONSOLE:ONLY
' Verification (m4-builtins-remaining, review of step 4a): $ERRORLOCATION:ON and a critical error in a statement
' of a $USELIBRARY top file (v58_root\libraries\includes\test\eloc\eloc_top.bi, an integer division by zero):
' the message names the line in that file and the file.
$ERRORLOCATION:ON
$USELIBRARY:'test/eloc'
PRINT "not reached"
SYSTEM
