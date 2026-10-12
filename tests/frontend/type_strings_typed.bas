' TEST: typed
$CONSOLE:ONLY
' STRING members (m2-defxxx-common-arrays task 8.1; measured, verification\v24_h_string_members, v24_x308): a
' member AS STRING beside others, in a nested type, in a variable, a static and a dynamic array, a local and a
' STATIC variable; stored, read, named with its suffix, passed by reference, a target of READ, MID$ and SWAP.
TYPE ts
    n AS STRING
    v AS LONG
    f AS STRING * 2
END TYPE
TYPE outer
    i AS ts
    t AS STRING
END TYPE
DIM p AS ts, o AS outer, a(2) AS ts
REDIM d(1) AS ts
p.n = "abc"
p.n = p.n + "!"
p.n$ = p.n$ + "?"
o.i.n = "in"
o.t = o.i.n + p.n
a(1).n = "one"
d(1).n = a(1).n
PRINT p.n; LEN(p.n); a(1).n; d(1).n; o.t
change p.n
change a(1).n
READ p.n, p.v
MID$(p.n, 2, 1) = "#"
SWAP p.n, o.t
ERASE a
REDIM _PRESERVE d(3) AS ts
ERASE d
SYSTEM
DATA text,5

SUB local
    DIM l AS ts
    STATIC k AS ts
    l.n = "x"
    k.n = k.n + l.n
END SUB
SUB change (s AS STRING)
    s = s + "?"
END SUB
