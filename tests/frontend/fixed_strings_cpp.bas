' TEST: cpp
$CONSOLE:ONLY
' The C++ of fixed-length strings (m2-numeric-types design D6, tasks 7.1 and 7.2), as the old compiler writes it: a
' variable is a fixed `qbs` over n bytes of its own, NUL-filled (`qbs_new_fixed(…,n,0)`, also each call for a local,
' DIVERGENCES-QB45.md Q-005); an element or member is read and stored through a temporary fixed `qbs` over its bytes
' (`qbs_new_fixed(…,n,1)`); stores are `qbs_set`, which cuts and pads; an array of them is n bytes per element; passed
' to a STRING parameter, the fixed `qbs` is the argument and the procedure's copy is written back through it (a
' FUNCTION call in a numeric statement then cleans the temporary up). Same output as qb64pe.exe.
TYPE rec
    id AS LONG
    nm AS STRING * 6
END TYPE
DIM SHARED sh AS STRING * 3
DIM f AS STRING * 4, r AS rec, a(2) AS STRING * 3, ra(1) AS rec
f = "ab": r.nm = "bob": a(1) = "x": ra(1).nm = "q": sh = "shared"
PRINT "["; f; "] ["; r.nm; "] ["; a(1); "] ["; ra(1).nm; "] ["; sh; "]"; ASC(a(0), 1); ASC(ra(0).nm, 1)
setlong f
setlong r.nm
setlong a(2)
PRINT "["; f; "] ["; r.nm; "] ["; a(2); "]"
n& = lenof(a(1)) + lenof(r.nm)
PRINT n&
loc
loc
SYSTEM

SUB loc
    DIM l AS STRING * 3
    STATIC t AS STRING * 3
    PRINT ASC(l, 1); ASC(t, 1)
    l = "zz": t = "s"
END SUB

SUB setlong (s AS STRING)
    s = "longer text"
END SUB

FUNCTION lenof& (s AS STRING)
    lenof = LEN(s)
END FUNCTION
