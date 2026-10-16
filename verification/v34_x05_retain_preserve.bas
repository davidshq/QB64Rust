$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (05_retain_preserve)
REDIM a(3) AS LONG
a(1) = 4
REDIM _RETAIN _PRESERVE a(5) AS LONG
PRINT a(1); UBOUND(a)
SYSTEM
