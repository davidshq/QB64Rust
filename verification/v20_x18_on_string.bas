$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): does the old compiler accept this? (on_string)
s$ = "a"
ON s$ GOTO l1
l1:
PRINT "l1"
