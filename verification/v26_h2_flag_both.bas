' Verification (step 11): the precompiler flag _CONSOLE_ when the program has $CONSOLE:ONLY and then $CONSOLE,
' as the upstream glut tests have on Windows.
$IF WINDOWS THEN
    $CONSOLE:ONLY
    PRINT "windows part"
$END IF
$CONSOLE
$IF _CONSOLE_ = 1 THEN
    PRINT "flag 1"
$ELSEIF _CONSOLE_ = 2 THEN
    PRINT "flag 2"
$ELSE
    PRINT "flag 0"
$END IF
_DEST _CONSOLE
PRINT "after _DEST _CONSOLE"
SYSTEM
