$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _MEMFILL with a value of a TYPE that holds a STRING.
TYPE ts
    n AS LONG
    s AS STRING
END TYPE
DIM w AS ts
DIM m AS _MEM, mw AS _MEM
w.n = 7: w.s = "abc"
mw = _MEM(w)
sz = mw.SIZE
m = _MEMNEW(sz * 2)
_MEMFILL m, m.OFFSET, sz * 2, w
PRINT sz; _MEMGET(m, m.OFFSET, LONG); _MEMGET(m, m.OFFSET + sz, LONG)
SYSTEM
