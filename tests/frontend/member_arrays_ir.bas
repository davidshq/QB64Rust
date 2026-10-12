' TEST: ir
$CONSOLE:ONLY
' Member arrays in the IR (m2-defxxx-common-arrays task 9.1, spec compiler/pipeline, "Member arrays in the IR"):
' the bounds are part of the user type; `x(0).n(1).q(2)` is one place, element 2 of member q of element 1 of
' member n of element 0 of x, each index a 64-bit integer, with no offset arithmetic; REDIM and ERASE of a
' member array are a Clear that names it, which may raise when an index stands on its way; a bound of one names
' it too; a whole member array is an array argument.
TYPE inner
    AS INTEGER q(2), r
END TYPE
TYPE outer
    AS inner n(1), m
END TYPE
TYPE t1
    id AS LONG
    v(0 TO 3) AS LONG
END TYPE
DIM t AS t1, a(2) AS t1, i AS LONG
REDIM x(0) AS outer
x(0).n(1).q(2) = 77
t.v(i) = x(0).n(1).q(2)
a(i).v(i + 1) = 5
i = UBOUND(a(i).v) + LBOUND(t.v, i)
REDIM a(i).v(0 TO 3)
ERASE t.v
fill a(i).v()
x(0).n(0) = x(0).n(1)
SYSTEM

SUB fill (b() AS LONG)
    b(0) = 1
END SUB
