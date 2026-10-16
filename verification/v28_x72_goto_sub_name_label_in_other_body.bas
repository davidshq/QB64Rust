$CONSOLE:ONLY
' Verification (m3-get-put-bload, task 1.2): A jump in a SUB to a name the main module has with a colon; the name is a SUB without parameters.
PRINT "ok"
SYSTEM
one:
PRINT "after one"
SUB one
END SUB
SUB two
    GOTO one
END SUB
