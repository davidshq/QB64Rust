$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (75_iif_type_values)
TYPE t
    n AS LONG
END TYPE
DIM r AS t, r2 AS t
r2 = _IIF(1, r, r2)
PRINT "ok"
SYSTEM
