' TEST: check-fail
$CONSOLE:ONLY
' Whole-array assignment, the forms the old compiler rejects (m3-whole-array-assign task 2.3, design D2), each a
' real error (measured, verification\v36_x01-x13, x15, x16, x18, x19, x21-x23, x26, x29, x34, x35, x41, x42,
' x45, x53-x55); a type suffix on both names compiles since m4-line-numbers-whole-arrays (x38).
TYPE t1
    x AS LONG
END TYPE
TYPE t2
    x AS LONG
END TYPE
TYPE m
    v(2) AS LONG
    w(2) AS INTEGER
    k AS LONG
END TYPE
CONST c = 1
DIM a(2) AS LONG, b(2) AS LONG, n AS LONG, i2(2) AS INTEGER, ul(2) AS _UNSIGNED LONG
DIM sg(2) AS SINGLE, db(2) AS DOUBLE, m2(2, 2) AS LONG, f4(2) AS STRING * 4, f8(2) AS STRING * 8
DIM st(2) AS STRING, u1(2) AS t1, u2(2) AS t2, b2(2) AS _BIT * 2, b3(2) AS _BIT * 3
DIM ub(2) AS _UNSIGNED _BIT, sb(2) AS _BIT, o(2) AS _OFFSET, q(2) AS _INTEGER64, x AS m, e(2) AS m
DIM a%(2), b%(2)
REDIM r(2) AS INTEGER
' "Expected array index after": one side a whole array, the other no array (x01, x03, x04, x13).
a() = n
n = a()
a() = 5
a() = b() + 1
' "Whole-array assignment requires empty parentheses on both sides" (x02, x26, x15, x29).
a() = b(1)
a(1) = b()
a() = undeclared()
a() = c()
' "Expected (...)": a FUNCTION (x18).
a() = f()
' "Cannot assign arrays of different types" (x05-x07, x10-x12, x19, x21-x23, x34, x35).
a() = i2()
a() = ul()
sg() = db()
st() = f4()
u1() = u2()
u1() = a()
x.v() = x.w()
b2() = b3()
sb() = ub()
o() = q()
u1() = e(1).v()
r() = a()
' "Cannot assign fixed-length STRING arrays with different element sizes" (x09).
f4() = f8()
' "Cannot assign arrays with different dimensions" (x08).
a() = m2()
' A suffix on one name only (x41, x42).
a() = b%()
a%() = b()
' The value in parentheses, as without them (x53, x54); a SUB as the source (x55: "Name already in use").
n = (a())
a() = (b(1))
a() = S()
SYSTEM
FUNCTION f& ()
    f& = 1
END FUNCTION
SUB S (p() AS LONG)
    DIM w(2, 2) AS LONG
    p(1) = 1
    ' The parameter's number of dimensions, from its other uses (x45).
    p() = w()
END SUB
