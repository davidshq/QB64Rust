' TEST: ir
$CONSOLE:ONLY
' REDIM _RETAIN in the IR (m3-redim-retain task 2.1, design D1; spec compiler/pipeline "Array storage in the IR"):
' an Allocate with the mode Retain, beside Redim and Preserve; no old descriptor, overlap or walk appears before
' the C++. REDIM _RETAIN of a member array gives no operation, as REDIM _PRESERVE of one (verification\v34_x09,
' x10): its path is not evaluated.
TYPE t
    v(1 TO 3) AS LONG
END TYPE
DIM x AS t, xs(2) AS t, k AS LONG
REDIM d(5) AS LONG
REDIM _RETAIN d(2 TO 9) AS LONG
REDIM _PRESERVE d(9) AS LONG
REDIM _RETAIN SHARED m(1 TO 3, 0 TO k) AS INTEGER
REDIM _RETAIN x.v(1 TO 3) AS LONG
REDIM _RETAIN xs(k).v(1 TO 3) AS LONG
PRINT d(2); m(1, 0); x.v(2)
