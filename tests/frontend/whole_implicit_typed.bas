' TEST: typed
$CONSOLE:ONLY
' The whole array x() of a name that is no array (m4-names-implicit-arrays task 3.3, design D5): the implicit array
' of one dimension, whose whole value is its first element and whose LEN is the bytes of its eleven elements, in
' the main module beside a scalar of the name, of other types, before an element of it, in an index of a store or of
' a READ target, which is a value, and in a SUB, where it is not allocated (measured, verification\v48_x04, x14,
' v55_b_implicit_whole, v55_x72, v55_x85).
x = 1
PRINT LEN(x()); x(); x
PRINT LEN(xd#()); "["; xs$(); "]"
PRINT LEN(y());
y(5) = 1
PRINT y(5); UBOUND(y)
DIM a(100)
a(LEN(w())) = 1
DATA 3
READ a(LEN(v#()))
PRINT a(44); a(88)
s
SYSTEM

SUB s
    PRINT LEN(u()); u()
END SUB
