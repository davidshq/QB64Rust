$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): $EXEICON, one form. A second one that names a missing file.
DIM l AS LONG
$EXEICON:'./v29_inc/icon.ico'
PRINT "ok"
SYSTEM
$EXEICON:'./v29_inc/missing.ico'
