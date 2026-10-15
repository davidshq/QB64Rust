$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): _INFLATE$ only in a SUB never called: still built and run.
PRINT "ok"
SYSTEM
SUB s
    PRINT _INFLATE$("x")
END SUB
