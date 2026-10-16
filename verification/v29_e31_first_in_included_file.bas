$CONSOLE:ONLY
' Verification (m3-sound, task 1.2): $EXEICON, one form. The first one inside an included file, a second after it:
' which line does the error name?
DIM l AS LONG
PRINT "ok"
'$INCLUDE:'v29_inc/exeicon.bi'
$EXEICON:'./v29_inc/icon.ico'
SYSTEM
