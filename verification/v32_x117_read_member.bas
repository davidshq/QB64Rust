$CONSOLE:ONLY
' Verification (m3-mem, by the review of the change): does the old compiler accept this? (117_read_member)
DIM m AS _MEM
READ m.SIZE
DATA 4
PRINT "ok"
SYSTEM
