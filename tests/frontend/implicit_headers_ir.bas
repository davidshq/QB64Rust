' TEST: ir
$CONSOLE:ONLY
' Below $DYNAMIC, an array used without DIM first named in the line of a block (m4-names-implicit-arrays task 4.3,
' design D2): before an IF, a FOR and a SELECT CASE the allocation is the first part of the header's statement; a
' WHILE and a DO tested at the top run it once, in a first test of their own; an ELSEIF, a test at LOOP and a CASE
' run it first in their test, each time (measured, verification\v24_x262, v55_a_implicit_headers).
DIM i AS LONG, k AS LONG
'$DYNAMIC
IF a(1) = 0 THEN PRINT 1
IF i = 0 THEN
    PRINT 2
ELSEIF c(1) = 0 THEN
    PRINT 3
END IF
FOR k = m(1) TO 1
NEXT
WHILE n(1) > 0
WEND
DO UNTIL r(1) = 0
LOOP
DO
LOOP UNTIL d(1) = 0
SELECT CASE g(1)
    CASE 1
    CASE e(1) + 5
END SELECT
SYSTEM
