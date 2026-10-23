' TEST: typed
$CONSOLE:ONLY
' A whole array taken as a value or a variable is its first element in memory, an element with no indexes
' (m4-line-numbers-whole-arrays, design D4; measured, `verification\v54_c_whole_values`, `v54_x05`-`x34`): in any
' expression, as the variable of READ, INPUT #, LINE INPUT #, SWAP, FIELD, LSET and RSET, and both sides of an
' assignment with a type suffix on both names.
DIM a(5 TO 7) AS LONG, b(2) AS LONG, s(2) AS STRING, t(2) AS STRING, x AS LONG
DIM a%(2), b%(2), c&(2)
PRINT a(); -a(); (a()); ABS(a()); a() + 1
IF a() THEN x = a() * 2
PRINT USING "##"; a(); _IIF(0, a(), b())
WRITE s()
READ a(), s()
INPUT #1, b()
LINE INPUT #1, s()
SWAP a(), b()
SWAP s(), t()
FIELD #1, 4 AS s()
LSET s() = "x": RSET t() = "y"
b%() = a%()
c&() = b%()
DATA 1, x
SYSTEM
