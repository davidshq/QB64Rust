$CONSOLE:ONLY
' Verification (m3-redim-retain, before its design): does the old compiler accept this? (15_retain_static_dim)
DIM a(5) AS LONG
a(1) = 3
REDIM _RETAIN a(9) AS LONG
PRINT a(1); UBOUND(a)
SYSTEM
