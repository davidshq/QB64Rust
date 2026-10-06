$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): an array element as the FOR variable.
DIM a(5)
FOR a(1) = 1 TO 3
PRINT a(1)
NEXT
PRINT a(1)
SYSTEM
