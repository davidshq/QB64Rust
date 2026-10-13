$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (13_lprint_forms)
IF 0 THEN LPRINT "a"; TAB(5); 1, 2
IF 0 THEN LPRINT USING "##"; 5
IF 0 THEN LPRINT
PRINT "ok"
SYSTEM
