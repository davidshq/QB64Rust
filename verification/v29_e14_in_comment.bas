$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): $EXEICON, one form. In a comment and after REM, with a file that is not there.
DIM l AS LONG
'$EXEICON:'./v29_inc/missing.ico'
REM $EXEICON:'./v29_inc/missing.ico'
PRINT "ok"
SYSTEM
