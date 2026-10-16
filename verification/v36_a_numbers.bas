$CONSOLE:ONLY
' Verification (m3-whole-array-assign, before its design): a() = b() of numbers: every numeric type, bounds
' that are not 0, two and three dimensions, a REDIM array on either side, the copy independent of its source,
' an array assigned to itself, the old values of the target overwritten, a suffix, the default type.
DIM i AS LONG, j AS LONG, k AS LONG
DIM sb(1 TO 3) AS _BYTE, db(1 TO 3) AS _BYTE
DIM sbu(1 TO 3) AS _UNSIGNED _BYTE, dub(1 TO 3) AS _UNSIGNED _BYTE
DIM si(1 TO 3) AS INTEGER, di(1 TO 3) AS INTEGER
DIM sui(1 TO 3) AS _UNSIGNED INTEGER, dui(1 TO 3) AS _UNSIGNED INTEGER
DIM sl(1 TO 3) AS LONG, dl(1 TO 3) AS LONG
DIM sul(1 TO 3) AS _UNSIGNED LONG, dul(1 TO 3) AS _UNSIGNED LONG
DIM s64(1 TO 3) AS _INTEGER64, d64(1 TO 3) AS _INTEGER64
DIM su64(1 TO 3) AS _UNSIGNED _INTEGER64, du64(1 TO 3) AS _UNSIGNED _INTEGER64
DIM ss(1 TO 3) AS SINGLE, ds(1 TO 3) AS SINGLE
DIM sd(1 TO 3) AS DOUBLE, dd(1 TO 3) AS DOUBLE
DIM sf(1 TO 3) AS _FLOAT, df(1 TO 3) AS _FLOAT
DIM so(1 TO 3) AS _OFFSET, dof(1 TO 3) AS _OFFSET
DIM sx(1 TO 3) AS _UNSIGNED _OFFSET, dx(1 TO 3) AS _UNSIGNED _OFFSET
FOR i = 1 TO 3
    sb(i) = -i: sbu(i) = 250 + i: si(i) = -1000 * i: sui(i) = 65530 + i: sl(i) = -100000 * i
    sul(i) = 4294967290 + i: s64(i) = -10000000000 * i: su64(i) = 18446744073709551610 + i
    ss(i) = i + 0.25: sd(i) = i + 0.125: sf(i) = i + 0.0625: so(i) = 1000 + i: dx(i) = 7: sx(i) = 2000 + i
    dl(i) = 99
NEXT
db() = sb(): dub() = sbu(): di() = si(): dui() = sui(): dl() = sl(): dul() = sul(): d64() = s64()
du64() = su64(): ds() = ss(): dd() = sd(): df() = sf(): dof() = so(): dx() = sx()
PRINT "1. every numeric type:"
FOR i = 1 TO 3
    PRINT db(i); dub(i); di(i); dui(i); dl(i); dul(i); d64(i); du64(i); ds(i); dd(i); df(i); dof(i); dx(i)
NEXT
PRINT "2. independent of its source:";
sl(2) = 5: PRINT dl(2); sl(2)
PRINT "3. two dimensions, m(-1 TO 1, 2 TO 4):"
DIM m(-1 TO 1, 2 TO 4) AS INTEGER, n(-1 TO 1, 2 TO 4) AS INTEGER
FOR i = -1 TO 1: FOR j = 2 TO 4: m(i, j) = 10 * i + j: NEXT: NEXT
n() = m()
FOR i = -1 TO 1: FOR j = 2 TO 4: PRINT n(i, j);: NEXT: PRINT: NEXT
PRINT "4. three dimensions:"
DIM c(1, 1, 1) AS DOUBLE, e(1, 1, 1) AS DOUBLE
FOR i = 0 TO 1: FOR j = 0 TO 1: FOR k = 0 TO 1: c(i, j, k) = 100 * i + 10 * j + k + 0.5: NEXT: NEXT: NEXT
e() = c()
FOR i = 0 TO 1: FOR j = 0 TO 1: FOR k = 0 TO 1: PRINT e(i, j, k);: NEXT: NEXT: NEXT: PRINT
PRINT "5. REDIM arrays, either side:";
REDIM r(2 TO 4) AS LONG, q(2 TO 4) AS LONG
DIM f(2 TO 4) AS LONG
FOR i = 2 TO 4: r(i) = i * 7: NEXT
q() = r(): f() = q()
FOR i = 2 TO 4: PRINT q(i); f(i);: NEXT: PRINT
PRINT "6. itself:";
r() = r()
FOR i = 2 TO 4: PRINT r(i);: NEXT: PRINT
PRINT "7. a suffix and the default type:";
DIM a%(2), b%(2), x(2), y(2)
a%(1) = 11: x(2) = 2.5
b%() = a%(): y() = x()
PRINT b%(1); y(2)
PRINT "8. LET:";
LET b%() = a%()
PRINT b%(1)
PRINT "9. after a colon and in a one-line IF:";
a%(0) = 3: b%(0) = 0
IF i > 0 THEN b%() = a%() ELSE PRINT "no"
PRINT b%(0)
SYSTEM
