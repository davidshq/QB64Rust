' TEST: check-fail
$CONSOLE:ONLY
' Arrays (m2-arrays-and-types task 4.3): the old compiler's errors as real errors (verification\v18_d_*, v18_e_*,
' v18_g_*), and what is not supported yet: implicit and whole arrays, arrays in SUBs, LBOUND of a non-array (dynamic
' arrays compile: redim_typed.bas, redim_errors.bas)
DIM x(10) AS LONG, y(3, 3)
x(1, 2) = 5
y(1) = 5
x("a") = 1
DIM x(10) AS LONG
DIM neg(-1)
DIM rev(5 TO 1)
DIM f(3)
PRINT LBOUND(x())
FOR x(1) = 1 TO 3: NEXT
z(3) = 4
x&(1) = 2
x!(1) = 2
PRINT LBOUND(scalar)
x() = 1
OPTION BASE 1
DIM c(3) AS LONG
c = 1
show
FUNCTION f (v)
    f = v
END FUNCTION
SUB show
    DIM t(5)
    STATIC u(5)
    PRINT x(1)
END SUB
