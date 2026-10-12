' TEST: check-ok
' The accepted forms of the console and window metacommands (m2-console-screenhide design D1, D2, task 2.1;
' verification\v26_x01, x04, x08, x15): any letter case, blanks before, twice, inside a SUB, after the last
' statement that runs. A tab or a colon before a preprocessor line in a branch that is not taken is skipped text
' (v26_x50, x57).
   $console
   $ScreenHide
$CONSOLE
$SCREENHIDE
$IF 0 THEN
	$ELSE
	$END IF
x = 1: $ELSE
x = 1: $END IF
$END IF
PRINT "to the window"
s
SYSTEM
$Console:Only
$CONSOLE

SUB s
    $CONSOLE
    $SCREENSHOW
    $SCREENHIDE
END SUB
