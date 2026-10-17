$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (38_on_key_in_sub)
setup
PRINT "ok"
SYSTEM
h:
RETURN
SUB setup
    ON KEY(1) GOSUB h
END SUB
