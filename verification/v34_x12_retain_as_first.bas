$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (12_retain_as_first)
REDIM a(3) AS LONG, b(3) AS LONG
a(2) = 7: b(3) = 8
REDIM _RETAIN AS LONG a(2 TO 5), b(3 TO 4)
PRINT a(2); b(3)
SYSTEM
