$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): DIM i AS LONG, FOR i closed by NEXT i& (the same variable).
DIM i AS LONG
FOR i = 1 TO 2
PRINT i
NEXT i&
SYSTEM
