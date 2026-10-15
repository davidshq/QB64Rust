' TEST: ir
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Whole-array assignment in the IR (m3-whole-array-assign task 2.1, design D3): one CopyArray per statement, from
' the source to the target, marked as raising, for two array variables, the source in parentheses, a member
' array to a variable and back, members of elements (the target's index found first), a dynamic member array,
' an array of an owner type, and an array parameter; no loop, descriptor or bounds check before the C++.
TYPE t
    v(1 TO 3) AS LONG
END TYPE
TYPE own
    d(1) _DYNAMIC AS LONG
    k AS LONG
END TYPE
DIM s(1 TO 3) AS LONG, d(1 TO 3) AS LONG, x AS t, e(2) AS t, i AS LONG
DIM p AS own, q AS own, os(1) AS own, od(1) AS own
d() = s()
LET d() = (s())
s() = x.v()
x.v() = d()
e(i + 1).v() = e(i).v()
q.d() = p.d()
od() = os()
fill d()
SYSTEM
SUB fill (a() AS LONG)
    DIM b(1 TO 3) AS LONG
    a() = b()
END SUB
