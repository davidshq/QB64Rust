$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (72_iif_string_condition)
PRINT _IIF("a", 1, 2)
PRINT "ok"
SYSTEM
