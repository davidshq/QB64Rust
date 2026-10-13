' TEST: ir
' Precompiler flags (m2-console-screenhide task 6.2, design D1; verification/v26_x26_flag_guarded_console): the
' only $CONSOLE stands inside the branch its own flag switches off. The first round meets it; the last round
' does not compile its line, and the program still has a console, by the flag: the Settings line says so, and
' there is no mark. _CONSOLE_ is 1 everywhere else. The window is not hidden: the line of $SCREENHIDE is not
' compiled either.
$IF _CONSOLE_ = 0 THEN
    $CONSOLE
    $SCREENHIDE
$END IF
_DEST _CONSOLE
$IF _CONSOLE_ = 1 THEN
    PRINT "flag 1"
$ELSE
    PRINT "flag not 1"
$END IF
SYSTEM
