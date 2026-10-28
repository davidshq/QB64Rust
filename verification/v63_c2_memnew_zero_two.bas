$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 9, two blocks of _MEMNEW(0) taken and freed in a row, after a
' block of 16 bytes was freed.
DIM m AS _MEM, n1 AS _MEM, n2 AS _MEM
m = _MEMNEW(16): _MEMFREE m
n1 = _MEMNEW(0)
_MEMFREE n1
n2 = _MEMNEW(0)
_MEMFREE n2
PRINT "after:"; _MEMEXISTS(n1); _MEMEXISTS(n2)
SYSTEM
