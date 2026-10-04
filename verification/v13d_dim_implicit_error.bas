$CONSOLE:ONLY
' Verification: a plain DIM x is an error ("Name already in use") when (x, SINGLE) already exists implicitly.
x = 1.5
DIM x
PRINT x
SYSTEM
