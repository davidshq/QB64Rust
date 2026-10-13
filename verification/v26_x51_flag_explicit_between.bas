$CONSOLE:ONLY
' Verification (step 11, task 1.4): $IF _EXPLICIT_ between two OPTION _EXPLICIT lines.
OPTION _EXPLICIT
$IF _EXPLICIT_ THEN
    PRINT "between: explicit 1"
$ELSE
    PRINT "between: explicit 0"
$END IF
OPTION _EXPLICIT
SYSTEM
