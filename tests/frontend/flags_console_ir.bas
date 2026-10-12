' TEST: ir
' Precompiler flags (m2-console-screenhide task 6.1, design D7; verification/v26_h_flag_console): _CONSOLE_ is 1
' on every line of a program with $CONSOLE, also above the line that sets it. The program is parsed twice: the
' first round reads it with the flag 0 and meets the setting, the second with the flag 1.
_DEST _CONSOLE
$IF _CONSOLE_ = 1 THEN
    PRINT "above the line: 1"
$ELSEIF _CONSOLE_ = 2 THEN
    PRINT "above the line: 2"
$ELSE
    PRINT "above the line: 0"
$END IF
$CONSOLE
$SCREENHIDE
$IF _CONSOLE_ = 1 THEN
    PRINT "below the line: 1"
$ELSE
    PRINT "below the line: not 1"
$END IF
SYSTEM
