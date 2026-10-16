$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (23_retain_preserve_name)
REDIM _RETAIN _preserve(3) AS LONG
PRINT "ok"
SYSTEM
