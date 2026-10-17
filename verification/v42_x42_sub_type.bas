$CONSOLE:ONLY
' Verification (m3-events, task 1.1): does the old compiler accept this? (42_sub_type)
TYPE t
    a AS LONG
END TYPE
ON TIMER(1) s 1
PRINT "ok"
SYSTEM
h:
RETURN
SUB s (n AS t)
END SUB
