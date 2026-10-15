$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (04_preserve_retain)
REDIM a(3) AS LONG
a(1) = 4
REDIM _PRESERVE _RETAIN a(5) AS LONG
PRINT a(1); UBOUND(a)
SYSTEM
