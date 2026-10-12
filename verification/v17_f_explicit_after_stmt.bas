$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT after another statement.
PRINT 1
OPTION _EXPLICIT
DIM x
x = 1
PRINT x
SYSTEM
