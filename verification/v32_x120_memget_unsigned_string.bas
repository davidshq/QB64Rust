$CONSOLE:ONLY
' Verification (m3-mem, by the review of the change): does the old compiler accept this? (120_memget_unsigned_string)
DIM m AS _MEM, s AS STRING
m = _MEMNEW(8)
s = _MEMGET(m, m.OFFSET, _UNSIGNED STRING * 4)
PRINT "ok"
SYSTEM
