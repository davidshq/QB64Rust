$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (08_key_forms)
ON KEY(1) s 7
KEY(1) ON
KEY(1) OFF
KEY(1) STOP
KEY ON
KEY OFF
KEY 15, CHR$(0) + CHR$(30)
KEY LIST
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS INTEGER)
END SUB
