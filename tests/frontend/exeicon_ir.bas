' TEST: ir
$CONSOLE:ONLY
' $EXEICON in the IR (m3-sound task 4.1, design D4; verification\v29_e*): where the line stands it is the
' statement _ICON with no argument, between the statements before and after it, and the settings line says the
' executable carries an icon and needs the icon part of the runtime. What follows the closing quote is dropped,
' a statement after a colon included (the PRINT of line 14 is not in the program). A line in a branch that is not
' taken does not count, and one in a comment is a comment. An _ICON statement stands beside it
$IF 0 THEN
$EXEICON:'./inc/missing.ico'
$END IF
'$EXEICON:'./inc/missing.ico'
REM $EXEICON:'./inc/missing.ico'
PRINT "before"
$EXEICON:'./inc/exeicon.ico': PRINT "same line"
PRINT "after"
_ICON
SYSTEM
