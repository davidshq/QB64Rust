$CONSOLE:ONLY
' Verification (SOMEDAY.md, the m3-mem row): _MEMPUT of a value of a TYPE that holds a STRING into a block of another such variable.
TYPE ts
    n AS LONG
    s AS STRING
END TYPE
DIM w AS ts, v AS ts
DIM m AS _MEM
w.n = 7: w.s = "abc"
m = _MEM(v)
_MEMPUT m, m.OFFSET, w
PRINT v.n; LEN(v.s); v.s
SYSTEM
