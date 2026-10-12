' TEST: cpp
$CONSOLE:ONLY
' Places in C++ (m2-arrays-and-types tasks 6.1–6.3): descriptors and static allocation of numeric, string and TYPE
' arrays (dimensions in reverse order), a TYPE variable's bytes (members in order, no padding, _FLOAT 32 bytes), the
' element store guarded after its index, a member store unguarded, a member-of-element store with the value first
' and the store skipped on a bad index (DIVERGENCES.md D-004), elements and members by reference, LBOUND/UBOUND,
' a local and a STATIC TYPE variable.
TYPE t
    m AS LONG
    f AS _FLOAT
    k AS INTEGER
END TYPE
DIM x(10) AS LONG, s(2) AS STRING
DIM p AS t, a(1 TO 3, 2) AS t
x(i) = 1
s(1) = "b"
p.k = 2
a(i, 0).k = x(3)
y = x(i) + p.m + a(2, i).k
bump x(3)
bump p.m
bump a(1, 1).m
PRINT LBOUND(a, 2); UBOUND(x); s(1)
show
SUB bump (v AS LONG)
    v = v + 1
END SUB
SUB show
    DIM q AS t
    STATIC r AS t
    q.k = r.k
END SUB
