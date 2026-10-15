$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.2): SYSTEM 4 in a SUB.
PRINT "x"
s
PRINT "not reached"
SYSTEM
SUB s
    SYSTEM 4
END SUB
