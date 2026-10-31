$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _MEMGET of a value of a TYPE that holds a STRING, from a block of another such variable; then the source's string is changed.
TYPE ts
    n AS LONG
    s AS STRING
END TYPE
DIM w AS ts, v AS ts
DIM m AS _MEM
w.n = 7: w.s = "abc"
m = _MEM(w)
_MEMGET m, m.OFFSET, v
PRINT v.n; LEN(v.s); v.s
w.s = "changed"
PRINT v.n; LEN(v.s); v.s
SYSTEM
