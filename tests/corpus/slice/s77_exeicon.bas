$CONSOLE:ONLY
' Slice (m3-sound, task 4.3; from verification\v29_e28_runs_in_place, e24): $EXEICON gives the executable the icon
' of the file it names, looked for in the folder of the main source file (the icon lies in s77_exeicon.files\ and
' is copied beside the program before it is compiled). Where the line stands it is the statement _ICON with no
' argument, which does nothing a console shows; what follows the closing quote is dropped, a statement after a
' colon included. That the executable holds the icon is checked by a command-line test of the driver.
PRINT "before"
$EXEICON:'./s77_exeicon.ico': PRINT "on the same line: never printed"
PRINT "after"
SYSTEM
