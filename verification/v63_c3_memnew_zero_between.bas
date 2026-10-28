$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 9, a block of _MEMNEW(0) freed between two blocks of 16 bytes,
' and a block taken after it.
DIM m1 AS _MEM, n AS _MEM, m2 AS _MEM, m3 AS _MEM
m1 = _MEMNEW(16)
n = _MEMNEW(0)
m2 = _MEMNEW(16)
_MEMFREE m1
_MEMFREE n
_MEMFREE m2
m3 = _MEMNEW(16)
_MEMPUT m3, m3.OFFSET, 42 AS LONG
PRINT "after:"; _MEMEXISTS(m1); _MEMEXISTS(n); _MEMEXISTS(m2); _MEMGET(m3, m3.OFFSET, LONG)
_MEMFREE m3
SYSTEM
