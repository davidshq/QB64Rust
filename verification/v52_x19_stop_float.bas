$CONSOLE:ONLY
' Verification (m4-qb64pe-sources, task 1.1): does the old compiler accept this? (x19_stop_float)
' STOP with a _FLOAT and with a variable not declared.
PRINT "a"
STOP 1.5## + q
SYSTEM
