$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (11_retain_list)
REDIM a(3) AS LONG, b(2) AS STRING
a(2) = 7: b(1) = "b"
REDIM _RETAIN a(1 TO 4) AS LONG, b(1 TO 3) AS STRING
PRINT a(2); b(1); UBOUND(b)
SYSTEM
