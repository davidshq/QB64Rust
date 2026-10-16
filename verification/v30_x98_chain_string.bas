$CONSOLE:ONLY
' Verification (step 4 of M3): does the old compiler accept this, and what does it do? (98_chain_string)
DIM f AS STRING
f = "v30_none"
IF 0 THEN CHAIN f
IF 0 THEN CHAIN "v30_" + "none"
IF 0 THEN CHAIN ("v30_none")
PRINT "ok"
SYSTEM
