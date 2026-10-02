$CONSOLE:ONLY
' Verification: _FLOAT suffix ## (type.bas:370 symboltype returns the DOUBLE code).
x## = 1
x## = x## / 3
PRINT x##; LEN(x##)
DIM y AS _FLOAT
y = 1: y = y / 3
PRINT y; LEN(y)
d# = 1: d# = d# / 3
PRINT d#; LEN(d#)
SYSTEM
