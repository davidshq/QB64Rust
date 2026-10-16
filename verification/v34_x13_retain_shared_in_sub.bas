$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (13_retain_shared_in_sub)
s
PRINT "ok"
SYSTEM
SUB s
    REDIM _RETAIN SHARED a(3) AS LONG
END SUB
