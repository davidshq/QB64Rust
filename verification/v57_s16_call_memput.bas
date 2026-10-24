$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): the stub CALL _MEMPUT without a type.
DIM m AS _MEM
DIM v AS LONG
m = _MEM(v)
CALL _MEMPUT(m, m.OFFSET, 5)
PRINT v
PRINT "ok"
SYSTEM
