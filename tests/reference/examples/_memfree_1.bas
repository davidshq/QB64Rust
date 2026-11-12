$CONSOLE:ONLY
DIM n AS _MEM, c AS _MEM
n = _MEMNEW(16)
c = n
_MEMFREE n
PRINT "After _MEMFREE, the block and its copy:"; _MEMEXISTS(n); _MEMEXISTS(c)
END
