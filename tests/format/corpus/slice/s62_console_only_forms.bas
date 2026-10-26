' Slice (m2-console-screenhide, group 2): the forms that make a program without a window. $CONSOLE:ONLY and
' $CONSOLE in one program, in any letter case, with blanks before, inside a SUB and after the last statement that
' runs. A PRINT above the first $CONSOLE:ONLY is not seen; each $CONSOLE:ONLY selects the console where it stands.
Print "above $CONSOLE:ONLY: not seen"
$Console:Only
Print "after $CONSOLE:ONLY"
$Console
Print "after $CONSOLE"
$ScreenHide
show
Print "after the SUB"
System
$ScreenShow
$Console
$Console:Only

Sub show
    $Console
    Print "in the SUB"
    $Console:Only
    Print "after $CONSOLE:ONLY in the SUB"
End Sub
