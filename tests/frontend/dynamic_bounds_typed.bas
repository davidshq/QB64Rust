' TEST: typed
$CONSOLE:ONLY
' Static or dynamic by the spelling of the bounds (m2-defxxx-common-arrays task 4.1; measured,
' verification\v24_b_constant_bounds and _2): one number, a minus before it included, or one CONST name, in any
' number of parentheses, is a constant bound and the array is static (no statement at its DIM). A bound with an
' operator makes a dynamic array though its value is known: the DIM is an Allocate. The arrays are named as in the
' two measurements: a1 to a20, then b1 to b8; b9 is v24_x249 (a minus before a hexadecimal number: one number).
CONST k = 5
CONST e = 2 + 3
DIM a1(5) AS LONG
DIM a2((5)) AS LONG
DIM a3(2 + 3) AS LONG
DIM a4(k) AS LONG
DIM a5(k + 1) AS LONG
DIM a6(-(2 ^ 2) TO 10 \ 3) AS LONG
DIM a7(5&) AS LONG
DIM a8(&H5) AS LONG
DIM a9(5.0) AS LONG
DIM a10(2.5) AS LONG
DIM a11(-1 TO 5) AS LONG
DIM a12(1 - 2 TO 5) AS LONG
DIM a13(1 TO k) AS LONG
DIM a14(k TO k * 2) AS LONG
DIM a15(5, 2 + 3) AS LONG
DIM a16(-5 TO -1) AS LONG
DIM a18(5%) AS LONG
DIM a19(1E1) AS LONG
DIM a20(NOT -6) AS LONG
DIM b1(-k TO 5) AS LONG
DIM b2((-1) TO 5) AS LONG
DIM b3(-(1) TO 5) AS LONG
DIM b4(((5))) AS LONG
DIM b5(k&) AS LONG
DIM b6(e) AS LONG
DIM b7(- 1 TO 5) AS LONG
DIM b8(--1 TO 5) AS LONG
DIM b9(-&H5 TO 5) AS LONG
' A variable, a FUNCTION and a built-in in a bound; OPTION BASE gives the lower bound of a dynamic array too, and
' the number 0 in plain digits keeps 0 to 0 in a dimension of one.
n = 4
DIM v1(n) AS LONG
DIM v2(1 TO twice(n), n) AS STRING
DIM v3(LEN("abc")) AS DOUBLE
OPTION BASE 1
DIM v4(n) AS LONG
DIM v5(0, n) AS LONG
SYSTEM
FUNCTION twice (x)
    twice = x * 2
END FUNCTION
