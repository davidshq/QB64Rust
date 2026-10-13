' TEST: ir
$SCREENHIDE
' Precompiler flags (m2-console-screenhide task 6.2, design D1; verification/v26_x38_flag_guarded_only): the
' only $CONSOLE:ONLY stands inside the branch its own flag switches off. _CONSOLE_ is 2 and the program has a
' console, by the flag; it also has a window, because the line that would take it away is not compiled, and the
' console is not selected there (no SelectConsole operation).
$IF _CONSOLE_ = 0 THEN
    $CONSOLE:ONLY
$END IF
$IF _CONSOLE_ = 2 THEN
    _ECHO "flag 2"
$ELSEIF _CONSOLE_ = 1 THEN
    _ECHO "flag 1"
$ELSE
    _ECHO "flag 0"
$END IF
SYSTEM
