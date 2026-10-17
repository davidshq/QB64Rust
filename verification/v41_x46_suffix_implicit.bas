$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (46_suffix_implicit)
a(1) = 1: b(1) = 2
_ARRAYCOPY a() TO b()
PRINT "ok"
SYSTEM
