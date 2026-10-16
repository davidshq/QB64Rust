' TEST: cpp
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Whole-array assignment in the emitter (m3-whole-array-assign task 3.1, design D4): the old compiler's block
' (AppendWholeArrayAssign) with its names, the descriptors target first: numbers, _BIT, STRING * n and a user type
' of numbers in one move of the bytes, STRING element by element, a user type with strings part by part, an owner
' type deep in place; a member array through a descriptor made for it and a dynamic member's own, where an
' index on the way makes the copy start from the pending error.
TYPE t
    v(1 TO 3) AS LONG
END TYPE
TYPE ts
    n AS INTEGER
    s AS STRING
END TYPE
TYPE own
    d(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
DIM s(1 TO 3) AS LONG, d(1 TO 3) AS LONG, bs(9) AS _BIT * 3, bd(9) AS _BIT * 3
DIM fs(2) AS STRING * 4, fd(2) AS STRING * 4, ss(2) AS STRING, sd(2) AS STRING
DIM ts1(1) AS ts, ts2(1) AS ts, x AS t, e(2) AS t, i AS LONG
DIM p AS own, q AS own, os(1) AS own, od(1) AS own
d() = s()
bd() = bs()
fd() = fs()
sd() = ss()
ts2() = ts1()
s() = x.v()
e(i).v() = d()
q.d() = p.d()
od() = os()
SYSTEM
