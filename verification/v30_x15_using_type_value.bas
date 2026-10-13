$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (15_using_type_value)
TYPE t
    n AS LONG
END TYPE
DIM r AS t
PRINT USING "##"; r
PRINT "ok"
SYSTEM
