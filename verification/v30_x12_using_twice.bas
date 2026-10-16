$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (12_using_twice)
PRINT "a"; USING "##"; 5; USING "###"; 6
PRINT "ok"
SYSTEM
