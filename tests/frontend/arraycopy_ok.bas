' TEST: check-ok
$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' _ARRAYCOPY, the forms the old compiler accepts (m3-arraycopy task 2.2; measured, verification\v41_a-v41_k,
' v41_x46, x53, x54, x57-x61, x63-x66, x70, x73-x75): the six forms, every element type it takes, member arrays
' of a plain TYPE and _DYNAMIC ones, array parameters, an array used without DIM, the places a statement stands.
TYPE t
    k AS INTEGER
    b(1 TO 6) AS LONG
    s(0 TO 2) AS STRING * 3
    v(0 TO 3) AS STRING
    g(1 TO 3, 1 TO 2) AS LONG
END TYPE
TYPE o
    inner AS t
END TYPE
TYPE dy
    _DYNAMIC n(1 TO 4) AS LONG
    _DYNAMIC s(0 TO 2) AS STRING
END TYPE
DIM s(3 TO 8) AS LONG, d(-2 TO 12) AS LONG, too(0 TO 9) AS LONG, arraycopy(1 TO 3) AS LONG
DIM m(1 TO 2, 0 TO 2) AS LONG, n(1 TO 2, 0 TO 2) AS LONG, big(0 TO 4, 0 TO 5) AS LONG
DIM g1(1 TO 3) AS _FLOAT, g2(0 TO 3) AS _FLOAT, of(1 TO 3) AS _UNSIGNED _OFFSET, of2(0 TO 3) AS _UNSIGNED _OFFSET
DIM fs(0 TO 2) AS STRING * 3, vs(1 TO 3) AS STRING, sm(1 TO 2, 1 TO 3) AS STRING, sn(1 TO 2, 1 TO 3) AS STRING
DIM v AS t, tt(1 TO 2) AS t, oo AS o, w(1 TO 2) AS dy, x AS dy, gg(1 TO 3, 1 TO 2) AS LONG
DIM b8 AS _BYTE, sg AS SINGLE
DIM SHARED sh(0 TO 9) AS LONG
' The six forms.
_ARRAYCOPY s() TO d()
_ARRAYCOPY s(4 TO 6) TO d(-1)
_ARRAYCOPY s() TO d(0)
_ARRAYCOPY s(4 TO 6) TO d()
_ARRAYCOPY big(1 TO 2, 2 TO 4) TO m(1, 0)
_ARRAYCOPY big(1 TO 2, 2 TO 4) TO n()
_ARRAYCOPY m() TO n()
_ARRAYCOPY m() TO big(1, 2)
' Kinds, strings, bounds of other types, written in other ways.
_ARRAYCOPY g1(2 TO 3) TO g2(1)
_ARRAYCOPY of() TO of2(1)
_ARRAYCOPY vs(1 TO 2) TO vs(2)
_ARRAYCOPY sm() TO sn()
_ARRAYCOPY sm(1 TO 2, 2 TO 3) TO sn(1, 1)
_ARRAYCOPY s(b8 TO sg) TO d(1.5)
_ARRAYCOPY s(LBOUND(s) TO UBOUND(s)) TO d(LBOUND(d))
_ARRAYCOPY s((3) TO ((4))) TO d(((0)))
_arraycopy   s( 3  to  4 )   to   s( 7 )
_ARRAYCOPY arraycopy() TO too()
_ARRAYCOPY s() TO too() ' trailing comment
_ARRAYCOPY s() TO too(): REM done
IF 1 THEN _ARRAYCOPY s() TO d() ELSE PRINT "no"
PRINT "x";: _ARRAYCOPY s() TO d()
' Member arrays of a plain TYPE and _DYNAMIC ones, nested and of elements.
_ARRAYCOPY s() TO v.b(2)
_ARRAYCOPY s(4 TO 5) TO tt(2).b(5)
_ARRAYCOPY v.b(2 TO 4) TO oo.inner.b()
_ARRAYCOPY v.s(1 TO 2) TO tt(1).s(0)
_ARRAYCOPY v.v() TO tt(2).v(1)
_ARRAYCOPY v.g(1 TO 2, 1 TO 2) TO gg(1, 1)
_ARRAYCOPY v.g() TO gg()
_ARRAYCOPY s(3 TO 6) TO w(2).n()
_ARRAYCOPY x.s() TO w(1).s(0)
' An array used without DIM (x46), parameters.
a(1) = 1: b(1) = 2
_ARRAYCOPY a() TO b()
cp s(), d()
fill s()
SYSTEM

SUB cp (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x() TO y(2)
END SUB

SUB fill (x() AS LONG)
    _ARRAYCOPY x() TO sh()
END SUB
