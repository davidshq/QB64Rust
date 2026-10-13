$CONSOLE:ONLY
' Verification (step 11): _EXPLICIT_ and _EXPLICITARRAY_ above an OPTION _EXPLICIT on the last line, and the
' flags nothing sets.
$IF _EXPLICIT_ THEN
    PRINT "explicit 1"
$ELSE
    PRINT "explicit 0"
$END IF
$IF _EXPLICITARRAY_ THEN
    PRINT "explicitarray 1"
$ELSE
    PRINT "explicitarray 0"
$END IF
$IF _ASSERTS_ OR _DEBUG_ OR _SOCKETS_ THEN
    PRINT "another flag is set"
$ELSE
    PRINT "asserts, debug, sockets 0"
$END IF
$IF _CONSOLE_ = 2 THEN
    PRINT "console 2"
$END IF
SYSTEM
OPTION _EXPLICIT
