' TEST: ir
$CONSOLE:ONLY
' Places in the IR (m2-arrays-and-types task 5.1, spec compiler/pipeline "Places lowered"): an element, a member, a
' member of an element, loads of each, an element and a member by reference, LBOUND/UBOUND. Indexes are I64; no
' descriptor slot, array_check or byte offset appears before the C++ output.
TYPE t
    m AS LONG
    n AS DOUBLE
END TYPE
DIM x(10) AS LONG, s(2) AS STRING
DIM p AS t, a(1 TO 3, 2) AS t
x(i) = 1
p.m = 2
a(i, 0).m = 3
y = x(i) + p.m + a(2, i).n
s(1) = s(0) + "b"
bump x(3)
bump a(1, 1).m
PRINT LBOUND(a, 2); UBOUND(x)
SUB bump (v AS LONG)
    v = v + 1
END SUB
