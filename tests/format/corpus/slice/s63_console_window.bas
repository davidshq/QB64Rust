' Slice (m2-console-screenhide, group 2): the first program with a console and a window. The window starts
' hidden, and the PRINT goes to it: nothing reaches standard output (_DEST _CONSOLE comes with group 3).
$Console
$ScreenHide
Print "to the hidden window"
System
