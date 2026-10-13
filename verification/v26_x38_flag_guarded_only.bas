' Verification (step 11): the only $CONSOLE:ONLY stands in the branch its own flag switches off. Is the program
' still one without a window (a PRINT with no _DEST seen), and what is _CONSOLE_?
$IF _CONSOLE_ = 0 THEN
    $CONSOLE:ONLY
$END IF
PRINT "printed with no _DEST"
$IF _CONSOLE_ = 2 THEN
    _ECHO "flag 2"
$ELSEIF _CONSOLE_ = 1 THEN
    _ECHO "flag 1"
$ELSE
    _ECHO "flag 0"
$END IF
_ECHO "dest:" + STR$(_DEST)
SYSTEM
