$CONSOLE:ONLY
' Verification (m3-mem, task 1.2): _MEMFREE of a block of _MEMNEW(0) after another block was freed. libqb never
' sets the type of a zero-size block's lock, so a lock taken again from the freed list keeps type 1 (malloc) and
' its old pointer, which _MEMFREE frees a second time.
DIM m AS _MEM, nm AS _MEM
m = _MEMNEW(16): _MEMFREE m
nm = _MEMNEW(0)
PRINT "before"
_MEMFREE nm
PRINT "after"
SYSTEM
