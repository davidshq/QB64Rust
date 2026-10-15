$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (18_retain_empty_parens)
REDIM a(3) AS LONG
REDIM _RETAIN a() AS LONG
PRINT "ok"
SYSTEM
