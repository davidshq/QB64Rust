$CONSOLE:ONLY
' Verification (m3-sound, task 1.1): $EXEICON, one form. Inside a SUB that is never called.
DIM l AS LONG
PRINT "ok"
SYSTEM
SUB one
    $EXEICON:'./v29_inc/icon.ico'
END SUB
