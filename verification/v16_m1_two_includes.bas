$CONSOLE:ONLY
' Verification (m2-parser-breadth, M1): two $INCLUDEs in one comment. Per the source only the last counts.
'$INCLUDE:'v16_inc_a.bi' $INCLUDE:'v16_inc_b.bi'
PRINT "after"
SYSTEM
