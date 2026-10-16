$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (03_shared_retain)
REDIM a(3) AS LONG
REDIM SHARED _RETAIN a(5) AS LONG
PRINT "ok"
SYSTEM
