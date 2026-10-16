' TEST: check-fail
$CONSOLE:ONLY
' Arrays (m2-arrays-and-types task 4.3): the old compiler's errors as real errors (verification\v18_d_*, v18_e_*,
' v18_g_*), `x() = 1` among them (whole arrays: whole_array_errors.bas; dynamic arrays: redim_typed.bas,
' redim_errors.bas; arrays used without DIM: implicit_arrays_typed.bas, implicit_arrays_errors.bas; arrays in
' procedures: proc_arrays_typed.bas, proc_arrays_errors.bas)
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
x&(1) = 2
x() = 1
OPTION BASE 1
DIM c(3) AS LONG
c = 1
FUNCTION f (v)
    f = v
END FUNCTION
