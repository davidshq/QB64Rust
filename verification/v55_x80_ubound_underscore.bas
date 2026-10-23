$CONSOLE:ONLY
' Verification (m4-names-implicit-arrays, task 1.1): does the old compiler accept this, and what does it do? (x80_ubound_underscore)
PRINT "ok"
SYSTEM
PRINT UBOUND(_foo)
