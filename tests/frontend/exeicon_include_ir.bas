' TEST: ir
$CONSOLE:ONLY
' $EXEICON in an included file, after the last statement that runs (m3-sound task 4.1; verification\v29_e12,
' e21, e25): the name, written with backslashes there, is looked for in the folder of the main source file, not
' of the included one
PRINT "ok"
SYSTEM
'$INCLUDE:'inc/exeicon.bi'
