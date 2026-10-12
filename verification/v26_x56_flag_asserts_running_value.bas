$CONSOLE:ONLY
$ASSERTS
' Verification (step 11, task 1.4): as v26_x39 for $ASSERTS. A setting inside $IF _ASSERTS_ = 0 below the line
' of $ASSERTS: is the OPTION _EXPLICIT met in the first round?
$IF _ASSERTS_ = 0 THEN
    OPTION _EXPLICIT
$END IF
$IF _EXPLICIT_ THEN
    PRINT "explicit 1"
$ELSE
    PRINT "explicit 0"
$END IF
$IF _ASSERTS_ THEN
    PRINT "asserts 1"
$ELSE
    PRINT "asserts 0"
$END IF
SYSTEM
