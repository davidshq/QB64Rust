$CONSOLE:ONLY
' Verification (m4-devices-continue-paths, task 1.1): DIM of a name with blanks around its dot.
DIM a . b AS LONG
a.b = 3
PRINT a.b; a . b
SYSTEM
