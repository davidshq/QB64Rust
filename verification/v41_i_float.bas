$CONSOLE:ONLY
' Verification (m3-arraycopy, while its proposal was written): _FLOAT arrays, whose copy in v41_b gave the wrong
' values: what each form copies, the size of an element, and element by element after a whole copy.
DIM i AS LONG
DIM g1(1 TO 4) AS _FLOAT, g2(1 TO 4) AS _FLOAT, g3(0 TO 7) AS _FLOAT
FOR i = 1 TO 4: g1(i) = i + 0.25: NEXT
PRINT "LEN of an element:"; LEN(g1(1))
_ARRAYCOPY g1() TO g2()
PRINT "whole to whole:";: FOR i = 1 TO 4: PRINT g2(i);: NEXT: PRINT
_ARRAYCOPY g1() TO g3(2)
PRINT "whole to start:";: FOR i = 0 TO 7: PRINT g3(i);: NEXT: PRINT
FOR i = 0 TO 7: g3(i) = 0: NEXT
_ARRAYCOPY g1(2 TO 3) TO g3(0)
PRINT "range to start:";: FOR i = 0 TO 7: PRINT g3(i);: NEXT: PRINT
DIM h1(1 TO 2, 1 TO 2) AS _FLOAT, h2(1 TO 2, 1 TO 2) AS _FLOAT
h1(1, 1) = 1.5: h1(2, 1) = 2.5: h1(1, 2) = 3.5: h1(2, 2) = 4.5
_ARRAYCOPY h1() TO h2()
PRINT "2D whole:"; h2(1, 1); h2(2, 1); h2(1, 2); h2(2, 2)
DIM m1 AS _MEM
m1 = _MEM(g1())
PRINT "_MEM element size:"; m1.ELEMENTSIZE; " size:"; m1.SIZE
_MEMFREE m1
SYSTEM
