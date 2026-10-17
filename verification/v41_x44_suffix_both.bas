$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (44_suffix_both)
DIM a%(1 TO 5), b%(0 TO 9)
_ARRAYCOPY a%() TO b%()
PRINT "ok"
SYSTEM
