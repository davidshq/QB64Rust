$CONSOLE:ONLY
' Verification (m3-sound, task 1.2): $EXEICON, one form. A second one in a $IF branch that is not taken.
DIM l AS LONG
$EXEICON:'./v29_inc/icon.ico'
$IF 0 THEN
$EXEICON:'./v29_inc/icon.ico'
$END IF
PRINT "ok"
SYSTEM
