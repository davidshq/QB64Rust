' TEST: cpp
$CONSOLE:ONLY
' The ASC statement in C++ (m4-qb64pe-sources task 3.2): the old compiler's inline code (qb64pe.bas 8410-8436),
' without and with the position, into a variable, a fixed-length string and an element
DIM a AS STRING, fx AS STRING * 4, arr(3) AS STRING, i AS LONG
ASC(a) = 65
ASC(a, i + 1) = 66
ASC(fx, 2) = i
ASC(arr(i)) = 67
