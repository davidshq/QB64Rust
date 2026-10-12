' TEST: check-fail
' A $CONSOLE in a $IF branch that is not taken does not count, and $SCREENHIDE gives no console: the program is
' one without a console, and the metacommands get no diagnostic (verification\v26_x06, x16).
$IF 0 THEN
    $CONSOLE
$END IF
$SCREENHIDE
PRINT "x"
SYSTEM
