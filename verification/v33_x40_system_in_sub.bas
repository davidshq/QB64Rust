$CONSOLE:ONLY
' Verification (m3-asserts-debug-zlib, task 1.1): does the old compiler accept this? (40_system_in_sub)
PRINT "ok"
SYSTEM
SUB s
    SYSTEM 3
END SUB
