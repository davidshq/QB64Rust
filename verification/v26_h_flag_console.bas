' Verification (step 11): the precompiler flag _CONSOLE_ before and after the line of $CONSOLE, and _DEST _CONSOLE
' above that line.
_DEST _CONSOLE
$IF _CONSOLE_ = 1 THEN
    PRINT "before the line: 1"
$ELSEIF _CONSOLE_ = 2 THEN
    PRINT "before the line: 2"
$ELSE
    PRINT "before the line: 0"
$END IF
$CONSOLE
$SCREENHIDE
$IF _CONSOLE_ = 1 THEN
    PRINT "after the line: 1"
$ELSEIF _CONSOLE_ = 2 THEN
    PRINT "after the line: 2"
$ELSE
    PRINT "after the line: 0"
$END IF
SYSTEM
