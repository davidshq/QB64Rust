$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (14_retain_as_name)
DIM x AS LONG
x = _RETAIN
PRINT x
SYSTEM
