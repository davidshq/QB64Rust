' TEST: ir
$CONSOLE
$SCREENHIDE
' Precompiler flags (m2-console-screenhide task 6.1, design D7; verification/v26_x39_flag_running_value): a
' setting inside a branch a line above has switched off. In the first round _CONSOLE_ is 0 at the first line
' and 1 from the line of $CONSOLE on, as in the old compiler's first pass, so the OPTION _EXPLICIT below is met
' in no round: _EXPLICIT_ is 0, and the variable that is not declared is no error.
$IF _CONSOLE_ = 0 THEN
    OPTION _EXPLICIT
$END IF
_DEST _CONSOLE
$IF _EXPLICIT_ THEN
    PRINT "explicit 1"
$ELSE
    PRINT "explicit 0"
$END IF
notDeclared = 1
SYSTEM
