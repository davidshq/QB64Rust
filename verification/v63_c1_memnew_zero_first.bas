$CONSOLE:ONLY
' Verification (libqb-fixes, task 1.2): study 39 row 9, _MEMFREE of a block of _MEMNEW(0) taken before any block was
' freed (a fresh lock, its type never set).
DIM nm AS _MEM
nm = _MEMNEW(0)
PRINT "exists:"; _MEMEXISTS(nm); " size:"; nm.SIZE
_MEMFREE nm
PRINT "after:"; _MEMEXISTS(nm)
SYSTEM
