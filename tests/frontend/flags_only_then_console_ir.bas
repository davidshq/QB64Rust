' TEST: ir
' Precompiler flags (m2-console-screenhide task 6.1; verification/v26_h2_flag_both): $CONSOLE:ONLY and below it
' $CONSOLE, as the upstream glut tests have: _CONSOLE_ is 1, the value asked for last, and the program has no
' window (the line of $CONSOLE:ONLY is compiled).
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
SYSTEM
