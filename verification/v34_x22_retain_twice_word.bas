$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (22_retain_twice_word)
REDIM a(3) AS LONG
REDIM _RETAIN _RETAIN a(5) AS LONG
PRINT "ok"
SYSTEM
