$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (16_retain_type_change)
REDIM a(3) AS LONG
REDIM _RETAIN a(5) AS INTEGER
PRINT "ok"
SYSTEM
