$SCREENHIDE
' Slice (m2-console-screenhide, group 6): the precompiler flags, third program. The only $CONSOLE:ONLY stands
' inside the branch its own flag switches off. _CONSOLE_ is 2 and the program has a console, by the flag; the
' line that would take the window away is not compiled in the last round, so the program keeps its (hidden)
' window: the destination is the window (0, not the console's handle), a PRINT goes there and is not seen, and
' _ECHO writes to the console.
$IF _CONSOLE_ = 0 THEN
    $CONSOLE:ONLY
$END IF
PRINT "printed to the window"
$IF _CONSOLE_ = 2 THEN
    _ECHO "flag 2"
$ELSEIF _CONSOLE_ = 1 THEN
    _ECHO "flag 1"
$ELSE
    _ECHO "flag 0"
$END IF
_ECHO "destination:" + STR$(_DEST) + STR$(_CONSOLE)
_DEST _CONSOLE
PRINT "after _DEST _CONSOLE"
SYSTEM
