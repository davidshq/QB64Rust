$CONSOLE:ONLY
' Verification (m4-statements-metacommands, task 1.1): the stub CALL _MEMGET.
DIM m AS _MEM
DIM v AS LONG
m = _MEM(v)
CALL _MEMGET(m, m.OFFSET, v)
PRINT v
PRINT "ok"
SYSTEM
