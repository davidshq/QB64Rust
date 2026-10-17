$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (41_str_whole_dims)
DIM s(1 TO 3) AS STRING, sn(0 TO 3, 0 TO 3) AS STRING
_ARRAYCOPY s() TO sn()
PRINT "ok"
SYSTEM
