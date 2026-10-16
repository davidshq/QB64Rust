$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): $EXEICON, one form. Between two PRINTs of a console program: does the line do anything when it is reached?
DIM l AS LONG
PRINT "before"
$EXEICON:'./v29_inc/icon.ico'
PRINT "after"
SYSTEM
