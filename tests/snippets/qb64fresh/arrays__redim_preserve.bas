$DYNAMIC
DIM arr(5) AS LONG
arr(0) = 42
arr(1) = 99
REDIM _PRESERVE arr(10) AS LONG
PRINT arr(0), arr(1)
