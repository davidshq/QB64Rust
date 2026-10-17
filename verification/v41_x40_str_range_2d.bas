$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (40_str_range_2d)
DIM sm(1 TO 2, 1 TO 2) AS STRING, sn(0 TO 3, 0 TO 3) AS STRING
_ARRAYCOPY sm(1 TO 2) TO sn(0)
PRINT "ok"
SYSTEM
