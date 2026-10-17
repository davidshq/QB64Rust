$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (42_str_nd_rank)
DIM sm(1 TO 2, 1 TO 2) AS STRING, s3(0 TO 3, 0 TO 3, 0 TO 1) AS STRING
_ARRAYCOPY sm(1 TO 2, 1 TO 2) TO s3(0, 0, 0)
PRINT "ok"
SYSTEM
