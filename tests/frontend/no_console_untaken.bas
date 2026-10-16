' TEST: ir
' A $CONSOLE in a $IF branch that is not taken does not count, and $SCREENHIDE gives no console: the program is
' one with a hidden window and no console, and the metacommands get no diagnostic (verification\v26_x06, x16;
' m3-window-output task 2.1)
$IF 0 THEN
    $CONSOLE
$END IF
$SCREENHIDE
PRINT "x"
SYSTEM
