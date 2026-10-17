' TEST: cpp
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' _ARRAYCOPY in C++ (m3-arraycopy design D5): the old compiler's block and libqb entry point for each form
' (1d_whole, 1d_range_at, 1d_whole_at, the range to the whole destination, nd_whole, nd_whole_at, nd_range_at and
' to the whole destination), _string for STRING, the size of an element for the others: STRING * 3, _OFFSET and
' _FLOAT by sizeof(long double) (DIVERGENCES.md D-025); a member array taken where its place is found, with the
' guard of D-004 where an index stands on the way, and a _DYNAMIC member's descriptor.
TYPE t
    b(1 TO 4) AS LONG
    s(0 TO 2) AS STRING
END TYPE
TYPE dy
    _DYNAMIC n(1 TO 4) AS LONG
END TYPE
DIM s(1 TO 5) AS LONG, d(0 TO 9) AS LONG, m(1 TO 2, 1 TO 3) AS LONG, n(0 TO 3, 0 TO 4) AS LONG
DIM vs(1 TO 3) AS STRING, ws(0 TO 4) AS STRING, fs(1 TO 3) AS STRING * 3, gs(0 TO 4) AS STRING * 3
DIM o(1 TO 3) AS _OFFSET, p(0 TO 4) AS _OFFSET, g1(1 TO 4) AS _FLOAT, g3(0 TO 7) AS _FLOAT
DIM v AS t, tt(1 TO 2) AS t, w AS dy, i AS LONG
_ARRAYCOPY s() TO d()
_ARRAYCOPY s(2 TO 4) TO d(i)
_ARRAYCOPY s() TO d(1)
_ARRAYCOPY s(2 TO 4) TO d()
_ARRAYCOPY m() TO n()
_ARRAYCOPY m() TO n(1, 1)
_ARRAYCOPY m(1 TO 2, 2 TO 3) TO n(0, 1)
_ARRAYCOPY m(1 TO 2, 2 TO 3) TO n()
_ARRAYCOPY vs(2 TO 3) TO ws(0)
_ARRAYCOPY vs() TO ws(1)
_ARRAYCOPY fs() TO gs(1)
_ARRAYCOPY o(1 TO 2) TO p(0)
_ARRAYCOPY g1(2 TO 3) TO g3(0)
_ARRAYCOPY s() TO v.b()
_ARRAYCOPY tt(i).b(1 TO 2) TO d(0)
_ARRAYCOPY vs() TO tt(2).s(0)
_ARRAYCOPY s(1 TO 4) TO w.n()
