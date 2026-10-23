' TEST: cpp
$CONSOLE:ONLY
' The C++ of a whole array taken as a value (m4-line-numbers-whole-arrays, design D4): the element at flat
' position 0 of the array's data, with no `array_check`, for a number, a STRING and a STRING * n array, read and
' stored; the offset of one (`VARPTR(v())`, `v54_x39`), on an array of its own, which `VARPTR` puts into
' conventional memory.
DIM a(5 TO 7) AS LONG, s(2) AS STRING, f(2) AS STRING * 4, v(2) AS LONG
PRINT a() + 1; s(); f()
PRINT VARPTR(v())
READ a()
LSET f() = "x"
DATA 3
SYSTEM
