$CONSOLE:ONLY
' Verification (m2-control-flow-slice, D1): a TYPE member as the FOR variable.
TYPE t
    v AS LONG
END TYPE
DIM r AS t
FOR r.v = 1 TO 2
PRINT r.v
NEXT
SYSTEM
