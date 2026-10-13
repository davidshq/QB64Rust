$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): $EXEICON, one form. A name with no folder that is not there.
DIM l AS LONG
$EXEICON:'missing.ico'
PRINT "ok"
SYSTEM
