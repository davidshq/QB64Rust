' Slice (m2-console-screenhide, group 2): the forms that make a program without a window. $CONSOLE:ONLY and
' $CONSOLE in one program, in any letter case, with blanks before, inside a SUB and after the last statement that
' runs. A PRINT above the first $CONSOLE:ONLY is not seen; each $CONSOLE:ONLY selects the console where it stands.
PRINT "above $CONSOLE:ONLY: not seen"
$CONSOLE:ONLY
PRINT "after $CONSOLE:ONLY"
$CONSOLE
PRINT "after $CONSOLE"
   $ScreenHide
show
PRINT "after the SUB"
SYSTEM
$SCREENSHOW
   $console
$Console:Only

SUB show
    $CONSOLE
    PRINT "in the SUB"
    $CONSOLE:ONLY
    PRINT "after $CONSOLE:ONLY in the SUB"
END SUB
