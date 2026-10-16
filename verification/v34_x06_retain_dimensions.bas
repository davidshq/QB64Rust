$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (06_retain_dimensions)
REDIM a(3) AS LONG
REDIM _RETAIN a(3, 3) AS LONG
PRINT "ok"
SYSTEM
