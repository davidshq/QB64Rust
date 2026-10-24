$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): LBOUND((a)) of a dynamic array.
REDIM a(3 TO 7) AS LONG
PRINT LBOUND((a)); UBOUND((a))
SYSTEM
