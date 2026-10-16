' TEST: cpp
$CONSOLE:ONLY
' REDIM _RETAIN in C++ (m3-redim-retain task 3.1, design D3; verification\v34_a, v34_b): beside _PRESERVE's
' locals, the old descriptor kept where the old block is captured, the overlap of each dimension after the size
' checks, and the old compiler's walk over the indexes, the first dimension fastest, with the copy of one element
' of each kind: memcpy for numbers and STRING * n, qbs_set for strings, setbits and getubits for _BIT (offsets
' counting elements), the parts of a user type with strings; and one array of three dimensions.
TYPE rec
    n AS LONG
    s AS STRING
    f AS STRING * 3
END TYPE
REDIM a(1 TO 5) AS LONG
REDIM _RETAIN a(3 TO 7) AS LONG
REDIM s(1 TO 4) AS STRING
REDIM _RETAIN s(3 TO 6) AS STRING
REDIM x(1 TO 3) AS STRING * 3
REDIM _RETAIN x(2 TO 5) AS STRING * 3
REDIM p(0 TO 9) AS _UNSIGNED _BIT * 5
REDIM _RETAIN p(5 TO 14) AS _UNSIGNED _BIT * 5
REDIM r(1 TO 3) AS rec
REDIM _RETAIN r(2 TO 4) AS rec
REDIM c(2, 2, 2) AS DOUBLE
REDIM _RETAIN c(1 TO 3, 0 TO 1, 1 TO 2) AS DOUBLE
PRINT a(3); s(3); x(2); p(5); r(2).n; c(1, 0, 1)
