$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): OPTION _EXPLICIT, DIM x AS LONG then x& (same variable) and x% (another).
OPTION _EXPLICIT
DIM x AS LONG
x& = 1
PRINT x
x% = 2
SYSTEM
