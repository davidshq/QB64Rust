' TEST: check-fail
$CONSOLE:ONLY
' _ARRAYCOPY, the forms the old compiler rejects (m3-arraycopy task 2.2, design D1-D3), each a real error
' (measured, verification\v41_x01-x45, x47-x52, x55, x56, x67-x69, x71, x72, x75, x76).
TYPE p
    x AS LONG
END TYPE
TYPE q
    x(1 TO 3) AS LONG
    z(1 TO 3) AS p
    w(1 TO 3) AS INTEGER
END TYPE
CONST k = 5
DIM a(1 TO 5) AS LONG, b(0 TO 9) AS LONG, c(0 TO 9) AS LONG, x AS LONG
DIM i2(0 TO 9) AS INTEGER, ul(0 TO 9) AS _UNSIGNED LONG, sg(1 TO 5) AS SINGLE, db(0 TO 9) AS DOUBLE
DIM st(1 TO 5) AS STRING, f4(0 TO 9) AS STRING * 4, f5(0 TO 9) AS STRING * 5
DIM u1(1 TO 5) AS p, u2(0 TO 9) AS p, bt(1 TO 5) AS _BIT, bt2(0 TO 9) AS _BIT
DIM b4(1 TO 5) AS _BIT * 4, b42(0 TO 9) AS _BIT * 4, mm(1 TO 5) AS _MEM, mm2(0 TO 9) AS _MEM
DIM m(1 TO 2, 1 TO 3) AS LONG, n(0 TO 3, 0 TO 4) AS LONG
DIM m3(1 TO 2, 1 TO 3, 1 TO 2) AS LONG, n3(0 TO 3, 0 TO 4, 0 TO 2) AS LONG
DIM sm(1 TO 2, 1 TO 2) AS STRING, sn(0 TO 3, 0 TO 3) AS STRING, s3(0 TO 3, 0 TO 3, 0 TO 1) AS STRING
DIM u(0 TO 5) AS STRING, a%(1 TO 5), b%(0 TO 9)
DIM o(1 TO 5) AS _OFFSET, uo(0 TO 9) AS _UNSIGNED _OFFSET, q64(1 TO 5) AS _INTEGER64
DIM v AS q, vs(1 TO 2) AS q, pv AS p
' The forms (x01-x10, x52, x67).
_ARRAYCOPY a() b()
_ARRAYCOPY a() TO b() TO c()
_ARRAYCOPY a()) TO b()
_ARRAYCOPY a( TO b()
_ARRAYCOPY TO b()
_ARRAYCOPY a() TO
_ARRAYCOPY a TO b()
_ARRAYCOPY a() TO b
_ARRAYCOPY a(1) TO b()
_ARRAYCOPY a(1 TO 2) TO b(1 TO 2)
_ARRAYCOPY v.x TO b()
_ARRAYCOPY (a()) TO b()
' Not an array (x11-x14, x44, x45, x47, x48, x51, x56).
_ARRAYCOPY x() TO b()
_ARRAYCOPY a() TO x()
_ARRAYCOPY zz() TO b()
_ARRAYCOPY a() TO zz()
_ARRAYCOPY a%() TO b%()
_ARRAYCOPY a&() TO b&()
_ARRAYCOPY fn() TO b()
_ARRAYCOPY k() TO b()
_ARRAYCOPY pv.x() TO b()
_ARRAYCOPY vs().x() TO b()
' Element types (x15-x24, x55, x68, x69, x72).
_ARRAYCOPY a() TO i2()
_ARRAYCOPY a() TO ul()
_ARRAYCOPY sg() TO db()
_ARRAYCOPY st() TO f4()
_ARRAYCOPY f4() TO st()
_ARRAYCOPY f4() TO f5()
_ARRAYCOPY u1() TO u2()
_ARRAYCOPY bt() TO bt2()
_ARRAYCOPY b4() TO b42()
_ARRAYCOPY mm() TO mm2()
_ARRAYCOPY v.z() TO u2()
_ARRAYCOPY o() TO uo()
_ARRAYCOPY q64() TO o()
_ARRAYCOPY v.w() TO b()
' Dimensions, ranges and starts (x25-x43).
_ARRAYCOPY m(1 TO 2) TO n(0)
_ARRAYCOPY a() TO n()
_ARRAYCOPY a() TO n(0, 0)
_ARRAYCOPY m() TO n(0)
_ARRAYCOPY a() TO b(0, 1)
_ARRAYCOPY a(1 TO 2, 1 TO 2) TO b(0, 0)
_ARRAYCOPY m3(1 TO 2, 1 TO 2) TO n3(0, 0, 0)
_ARRAYCOPY m(1 TO 2, 1 TO 2) TO n(0)
_ARRAYCOPY m(1 TO 2, 1 TO 2) TO n(0, 0, 0)
_ARRAYCOPY m(1 TO 2, 3) TO n(0, 0)
_ARRAYCOPY m(1 TO 2 TO 3, 1 TO 2) TO n(0, 0)
_ARRAYCOPY m(1 TO 2, 1 TO 2) TO n(0, )
_ARRAYCOPY m(1 TO 2, ) TO n(0, 0)
_ARRAYCOPY a(1 TO 2, 1 TO 2) TO b()
_ARRAYCOPY m(1 TO 2, 1 TO 2) TO n3(0, 0, 0)
_ARRAYCOPY sm(1 TO 2) TO sn(0)
_ARRAYCOPY st() TO sn()
_ARRAYCOPY sm(1 TO 2, 1 TO 2) TO s3(0, 0, 0)
_ARRAYCOPY sm() TO u(0)
' A bound or start that is a string (x49, x50).
_ARRAYCOPY a("1" TO 2) TO b(0)
_ARRAYCOPY a(1 TO 2) TO b("0")
' Array parameters: their number of dimensions comes from the calls, after every body is checked (x71, x75, x76).
DIM p2(1 TO 2, 1 TO 2) AS LONG, q2(1 TO 2, 1 TO 2) AS LONG, one(1 TO 5) AS LONG
fill p2(), q2()
cp p2(), one()
SYSTEM

FUNCTION fn&
    fn& = 1
END FUNCTION

SUB fill (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x() TO y(1)
END SUB

SUB never (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x(1 TO 2, 1 TO 2) TO y(1, 1)
END SUB

SUB cp (x() AS LONG, y() AS LONG)
    _ARRAYCOPY x() TO y(2)
END SUB

' STRING parameters that nothing gives a number of dimensions take the same message, whatever the form.
SUB nevers (x() AS STRING, y() AS STRING)
    _ARRAYCOPY x() TO y()
END SUB

SUB neverr (x() AS STRING, y() AS STRING)
    _ARRAYCOPY x(1 TO 2, 1 TO 2) TO y(1, 1)
END SUB
