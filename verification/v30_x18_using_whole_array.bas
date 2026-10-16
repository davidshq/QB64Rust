$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (18_using_whole_array)
DIM a(3) AS LONG
PRINT USING "##"; a()
PRINT "ok"
SYSTEM
