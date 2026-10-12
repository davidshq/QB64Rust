' TEST: typed
$CONSOLE:ONLY
' Dynamic arrays in the main module (m2-defxxx-common-arrays task 4.1; measured, verification\v24_b_*, v24_c_*,
' v24_d_*, v24_x39 to x56, x86, x98, x99). What an array is, is settled by its first declaration; every later DIM or
' REDIM of it is an Allocate.
n = 3
' REDIM declares the array; REDIM again and REDIM _PRESERVE give it new bounds.
REDIM r(5) AS LONG
REDIM r(1 TO n) AS LONG
REDIM _PRESERVE r(n + 1) AS LONG
' Without a type a REDIM keeps the type of an array declared with AS, by REDIM or by DIM; a suffix names the same
' array or another one.
REDIM r(7)
REDIM r&(8)
REDIM r!(9)
DIM d(n) AS INTEGER
REDIM d(2)
' Two arrays in one statement, and the type before the names.
REDIM p1(2), p2(3, n) AS STRING
REDIM AS DOUBLE q1(2), q2(1 TO 2)
' DIM of a dynamic array, with constant bounds too, and DIM with a variable bound of a static one: both run (and
' raise error 10).
DIM d(5) AS INTEGER
DIM s(5) AS LONG
DIM s(n) AS LONG
REDIM s(9) AS LONG
REDIM _PRESERVE s(9) AS LONG
' Below $DYNAMIC a DIM with constant bounds makes a dynamic array; below $STATIC a static one again.
'$DYNAMIC
DIM m1(5) AS LONG
DIM SHARED m2(1 TO 2, 3) AS STRING * 4
'$STATIC
DIM m3(5) AS LONG
' SHARED after _PRESERVE; the array is seen in a SUB, where a REDIM and an ERASE reach it.
REDIM SHARED g1(2) AS LONG
REDIM _PRESERVE SHARED g2(2) AS LONG
' Arrays named preserve and retain: the words without their underscore are names (measured, v24_x254).
REDIM preserve(3) AS LONG
REDIM retain(4) AS LONG
' SHARED on a later REDIM of an array is accepted and changes nothing (measured, v24_x248).
REDIM ns(3) AS LONG
REDIM SHARED ns(4) AS LONG
' With nothing between the parentheses, and without parentheses, DIM and REDIM declare a scalar.
DIM e1() AS LONG
REDIM e2() AS LONG
REDIM e3 AS LONG
e1 = 1.5: e2 = 2.5: e3 = 3.5
' A typed array beside a scalar of its name typed alike.
DIM e1(3) AS LONG
' ERASE clears a static array and releases a dynamic one, in order.
ERASE s, r, m3
ERASE r&
r(1) = d(2) + UBOUND(p2, 2)
grow
SYSTEM
SUB grow
    REDIM g1(4) AS LONG
    REDIM _PRESERVE g2(4)
    REDIM m2(1 TO 3, 3)
    ERASE g1, m2
END SUB
