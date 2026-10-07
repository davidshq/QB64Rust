' TEST: typed
$CONSOLE:ONLY
' Static arrays (m2-arrays-and-types task 4.3): bounds from constants and constant expressions (floats rounded half
' to even), an array beside a scalar of the same name, indexes converted to _INTEGER64 (a float rounded half to
' even), string arrays, elements by reference and by copy, LBOUND/UBOUND typed _INTEGER64
CONST hi = 4
DIM x(10) AS LONG, m(1 TO 3, -2 TO hi) AS INTEGER
DIM s$(2.5), f(-(2 ^ 2) TO 10 \ 3)
DIM SHARED g(3) AS DOUBLE
DIM a(3)
a = 5
a(1) = a
x(1.5) = x(2) + 1
m(2, k%) = 7
s$(1) = s$(0) + "b"
PRINT LBOUND(x); UBOUND(m, 2); UBOUND(m, 1.5)
bump x(3)
bump (x(3))
bump m(1, 1)
SUB bump (v AS LONG)
    g(1) = v
END SUB
