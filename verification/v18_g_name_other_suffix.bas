$CONSOLE:ONLY
' Verification (m2-arrays-and-types, D1): an array declared AS LONG used with another suffix.
DIM c(3) AS LONG
c!(1) = 1.5
PRINT c(1); c!(1)
SYSTEM
