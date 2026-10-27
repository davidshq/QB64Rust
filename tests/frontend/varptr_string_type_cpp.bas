' TEST: cpp
$CONSOLE:ONLY
' A TYPE variable and a static array of a TYPE that holds a STRING, in conventional memory (m4-left-marked task
' 4.1, design D3): the variable is allocated from dblock, its strings made after the memset with the trace of
' QB64PE-DIVERGENCES.md D-037; the array's elements get their strings as the old compiler makes them (v60_d5).
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
DIM r AS ts, e(2) AS ts
PRINT VARPTR(r.v); VARPTR(e(1).v)
r.n = "abc"
PRINT r.n; LEN(r.n)
SYSTEM
