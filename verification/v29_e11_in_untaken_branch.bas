$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): $EXEICON, one form. A missing file in a $IF branch that is not taken.
DIM l AS LONG
$IF 0 THEN
$EXEICON:'./v29_inc/missing.ico'
$END IF
PRINT "ok"
SYSTEM
