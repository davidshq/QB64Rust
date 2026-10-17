$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (01_sub_gosub_main)
setup
PRINT "ok"
SYSTEM
h:
RETURN
SUB setup
    ON TIMER(1) GOSUB h
END SUB
