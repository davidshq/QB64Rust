$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (24_retain_fixed_dims_two_to_two)
REDIM a(1, 1) AS LONG
a(1, 1) = 4
REDIM _RETAIN a(1 TO 2, 1 TO 2) AS LONG
PRINT a(1, 1)
SYSTEM
