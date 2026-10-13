$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (94_min_type_value)
TYPE t
    n AS LONG
END TYPE
DIM r AS t
PRINT _MIN(r, 1)
PRINT "ok"
SYSTEM
