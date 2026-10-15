$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (08_retain_lower)
redim a(3) as long
a(1) = 4
redim _retain a(1 to 5) as long
PRINT a(1); UBOUND(a)
SYSTEM
