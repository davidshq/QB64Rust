$CONSOLE:ONLY
' Verification (step 11, task 1.4): the only OPTION _EXPLICIT stands in the branch its own flag switches off.
' What is _EXPLICIT_? (v26_x55: is the option in force?)
$IF _EXPLICIT_ = 0 THEN
    OPTION _EXPLICIT
$END IF
$IF _EXPLICIT_ THEN
    PRINT "explicit 1"
$ELSE
    PRINT "explicit 0"
$END IF
SYSTEM
