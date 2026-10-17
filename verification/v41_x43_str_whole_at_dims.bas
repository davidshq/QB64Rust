$CONSOLE:ONLY
' Verification (m3-arraycopy, before its proposal): does the old compiler accept this? (43_str_whole_at_dims)
DIM sm(1 TO 2, 1 TO 2) AS STRING, u(0 TO 5) AS STRING
_ARRAYCOPY sm() TO u(0)
PRINT "ok"
SYSTEM
