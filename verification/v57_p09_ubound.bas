$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): UBOUND((b)) and UBOUND((b), 1).
DIM b(2 TO 5, 3) AS INTEGER
PRINT UBOUND((b))
PRINT UBOUND((b), 1)
PRINT UBOUND((b), 2)
SYSTEM
