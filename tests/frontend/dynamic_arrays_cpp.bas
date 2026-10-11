' TEST: cpp
$CONSOLE:ONLY
' Dynamic arrays in the emitter (m2-defxxx-common-arrays task 4.2): the descriptor of a dynamic array is made "not
' allocated" (maindata.txt); Allocate in each mode, for numbers and for strings (error 10 for a static array, in
' mode Dim for an allocated one; the bounds into a descriptor of their own, error 5 for no element without leaving,
' error 257 for what cannot be held; the new block before the old one is released; the descriptor written last);
' REDIM _PRESERVE copies by flat position (strings assigned, bytes copied); Clear and Release; a bound that uses a
' string is cleaned up; clear.txt releases a dynamic array.
n = 4
DIM d(n, 1) AS LONG
REDIM d(1 TO n, 2) AS LONG
REDIM _PRESERVE d(1 TO n, 5) AS LONG
REDIM t(n) AS STRING
REDIM _PRESERVE t(LEN("ab" + t(0))) AS STRING
DIM s(5) AS LONG
REDIM s(n) AS LONG
d(1, 1) = UBOUND(d, 2) + LEN(t(n))
ERASE s, d, t
SYSTEM
