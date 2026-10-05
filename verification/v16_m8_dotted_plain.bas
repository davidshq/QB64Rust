$CONSOLE:ONLY
' Verification (m2-parser-breadth, M8): a.b as a plain variable name, no TYPE anywhere.
a.b = 3
a = 4
PRINT a.b; a
SYSTEM
