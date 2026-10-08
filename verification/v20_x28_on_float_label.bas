$CONSOLE:ONLY
' Verification (m2-core-builtins, D1): does the old compiler accept this? (on_float_label)
ON 1.5 GOTO l1, l2
l1:
PRINT "l1"
l2:
PRINT "l2"
SYSTEM
