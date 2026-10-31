$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _MEMPUT of a STRING member; one form, on the last line, as code that is not reached.
TYPE ts
    n AS LONG
    s AS STRING
END TYPE
DIM t AS ts
DIM m AS _MEM
PRINT "ok"
SYSTEM
_MEMPUT m, m.OFFSET, t.s
