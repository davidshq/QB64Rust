' TEST: typed
$CONSOLE:ONLY
' OPTION BASE (m2-defxxx-common-arrays task 3.2; verification\v24_e_option_base, v24_x69 to x85): the lower bound
' of a dimension written without TO, from the statement on in the file: an array above it, below it, bounds with
' TO (unchanged), the number 0 in plain digits (0 to 0, per dimension, also written 00), base 0 again, a
' statement skipped by GOTO and one inside a SUB (both apply to the lines below them), what follows the word
' after BASE (ignored), $STATIC in a comment (a static array as before), and 0 or 1 with zeros before it (01,
' v24_x239, x240, x243)
DIM above(3)
OPTION BASE 1
DIM a(3), b(2 TO 4)
DIM z(0), z2(0, 3), z3(3, 0), z4(00)
OPTION BASE 0
DIM c(3)
GOTO skip
OPTION BASE 1
skip:
DIM e(3)
OPTION BASE 0 + 1
DIM f(3)
OPTION BASE 1 x
DIM g(3)
'$STATIC
DIM h(3 TO 5, 2)
OPTION BASE 01
DIM i(3)
OPTION BASE 00
DIM j(3)
OPTION BASE 001
DIM k(3)
OPTION BASE 0
a(1) = LBOUND(above) + UBOUND(z2, 2)
SYSTEM
SUB setbase
    OPTION BASE 0
END SUB
