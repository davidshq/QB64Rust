$CONSOLE:ONLY
' Verification (m2-parser-breadth, M8): DIM of a dotted name, and a dotted name with a suffix.
DIM x.y AS LONG
x.y = 7
x.z$ = "s"
PRINT x.y; x.z$
SYSTEM
