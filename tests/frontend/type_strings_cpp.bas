' TEST: cpp
$CONSOLE:ONLY
' STRING members in the emitter (m2-defxxx-common-arrays task 8.1, design D9): a STRING member is the pointer to
' its string in the value's bytes. Every value of such a type gets its strings when it is made (a variable and a
' static array in maindata.txt, a local in dataK.txt, a dynamic array in its allocation) and loses them when it
' goes (mainfree.txt, freeK.txt, a release, the old block of a REDIM); ERASE of a static array empties them
' (clear.txt does the same), REDIM _PRESERVE copies the kept elements part by part. A store into a member of an
' element copies the value first and is skipped after an index outside (QB64PE-DIVERGENCES.md D-004).
TYPE ts
    n AS STRING
    v AS LONG
END TYPE
TYPE outer
    k AS INTEGER
    i AS ts
    t AS STRING
END TYPE
DIM p AS ts, o AS outer, a(2) AS ts, i AS LONG
p.n = "abc"
o.i.n = p.n + "!"
a(i).n = o.i.n
PRINT a(i).n; LEN(p.n)
change a(1).n
ERASE a
REDIM d(1) AS outer
REDIM _PRESERVE d(3) AS outer
ERASE d
local
SYSTEM

SUB local
    DIM l AS ts
    STATIC k AS ts
    DIM la(i) AS ts
    l.n = "x"
    k.n = k.n + l.n
END SUB
SUB change (s AS STRING)
    s = s + "?"
END SUB
