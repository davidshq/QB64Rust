$CONSOLE:ONLY
$UNSTABLE:TYPEFIELDS
' Verification (wasm-pointer-width, design D5 and D8): members that are pointers. A member array of STRING
' between other members, in a variable and in an array element; LEN of it (W bytes an element); and a TYPE that
' owns two _DYNAMIC member arrays with an _OFFSET between them, copied whole, with its size from _MEM (five
' slots of W bytes: the old compiler aligns each dynamic member to a pointer and rounds the size up to one).
$IF 32BIT THEN
    CONST W = 4
$ELSE
    CONST W = 8
$END IF
TYPE words
    n AS INTEGER
    w(0 TO 2) AS STRING
    tail AS LONG
END TYPE
TYPE owner
    a AS INTEGER
    _DYNAMIC d(1 TO 2) AS LONG
    p AS _OFFSET
    _DYNAMIC e(1 TO 2) AS STRING
    z AS _BYTE
END TYPE
DIM v AS words, both(1) AS words, o AS owner, o2 AS owner, m AS _MEM
v.n = 3: v.w(0) = "zero": v.w(1) = "one": v.w(2) = "two": v.tail = 99
PRINT v.n; v.w(0); v.w(1); v.w(2); v.tail
PRINT LEN(v.w()) = 3 * W
both(1) = v
both(1).w(1) = "uno"
PRINT both(1).w(0); both(1).w(1); both(1).w(2); both(1).tail; v.w(1); "<"; both(0).w(2); ">"; both(0).tail
o.a = 1: o.d(1) = 10: o.d(2) = 20: o.p = 30: o.e(1) = "x": o.e(2) = "yy": o.z = 4
REDIM _PRESERVE o.d(1 TO 4) AS LONG
o.d(4) = 40
o2 = o
o2.e(2) = "changed": o2.d(1) = 11: o2.p = 31
PRINT o.a; o.d(1); o.d(2); o.d(4); o.p; o.e(1); o.e(2); o.z
PRINT o2.a; o2.d(1); o2.d(2); o2.d(4); o2.p; o2.e(1); o2.e(2); o2.z
m = _MEM(o)
PRINT m.SIZE = 5 * W
SYSTEM
