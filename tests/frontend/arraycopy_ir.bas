' TEST: ir
$CONSOLE:ONLY
' _ARRAYCOPY in the IR (m3-arraycopy design D4): one ArrayCopy per statement, the source's ranges and the
' destination's starts as I64 values converted as an index is, in the statement's order; the source's place before
' the destination's; a member array of an element, of two dimensions; an array parameter.
TYPE t
    b(1 TO 4) AS LONG
    g(1 TO 2, 1 TO 3) AS LONG
END TYPE
DIM s(1 TO 5) AS LONG, d(0 TO 9) AS LONG, m(1 TO 2, 1 TO 3) AS LONG, n(0 TO 3, 0 TO 4) AS LONG
DIM vs(1 TO 3) AS STRING, ws(0 TO 4) AS STRING, tt(1 TO 2) AS t, i AS INTEGER
_ARRAYCOPY s() TO d()
_ARRAYCOPY s(2 TO 4) TO d(f(1))
_ARRAYCOPY s() TO d(i)
_ARRAYCOPY s(1.5 TO 3.5) TO d()
_ARRAYCOPY m(1 TO 2, 2 TO 3) TO n(0, 1)
_ARRAYCOPY m(1 TO 2, 2 TO 3) TO n()
_ARRAYCOPY m() TO n(1, 1)
_ARRAYCOPY vs(2 TO 3) TO ws(0)
_ARRAYCOPY tt(f(1)).b(2 TO 3) TO tt(f(2)).b(1)
_ARRAYCOPY tt(1).g() TO n(1, 1)
cp s(), d()

SUB cp (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x() TO y(2)
END SUB

FUNCTION f& (k AS LONG)
    f& = k
END FUNCTION
