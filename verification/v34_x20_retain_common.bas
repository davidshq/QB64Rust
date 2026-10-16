$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (20_retain_common)
COMMON SHARED a() AS LONG
REDIM a(1 TO 3) AS LONG
a(3) = 6
REDIM _RETAIN a(3 TO 4) AS LONG
PRINT a(3)
SYSTEM
