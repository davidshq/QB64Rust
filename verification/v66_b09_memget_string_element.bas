$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _MEMGET into an element of an array of variable-length strings; one form, on the last line, as code that is not reached.
DIM s(3) AS STRING
DIM m AS _MEM
PRINT "ok"
SYSTEM
_MEMGET m, m.OFFSET, s(1)
