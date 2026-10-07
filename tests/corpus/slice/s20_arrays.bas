$CONSOLE:ONLY
' Slice program (m2-arrays-and-types, D9): static arrays. Bounds, element read and write, every slice type,
' string arrays, LBOUND and UBOUND, elements passed by reference, DIM SHARED. No PRINT comma.
CONST lo = -3, hi = 4
DIM b(-2 TO 3, 5) AS LONG
DIM z(0) AS INTEGER
DIM w(lo TO hi) AS DOUBLE
DIM f(2.5) AS LONG
DIM g(1.5 TO 3.5) AS SINGLE
DIM e(-(2 ^ 2) TO 10 \ 3) AS _INTEGER64
DIM SHARED s(5) AS LONG
DIM names(3) AS STRING
DIM t$(2)

PRINT "initial:"; b(0, 0); z(0); w(lo); f(2); g(2); e(-4); "["; names(1); "]"; s(5)
b(-2, 0) = 1: b(3, 5) = 2: b(0, 3) = 3
PRINT "b:"; b(-2, 0); b(3, 5); b(0, 3); b(1, 3)
PRINT "LBOUND/UBOUND b:"; LBOUND(b); UBOUND(b); LBOUND(b, 1); UBOUND(b, 1); LBOUND(b, 2); UBOUND(b, 2)
PRINT "z:"; LBOUND(z); UBOUND(z)
PRINT "w:"; LBOUND(w); UBOUND(w)
PRINT "f:"; LBOUND(f); UBOUND(f)
PRINT "g:"; LBOUND(g); UBOUND(g)
PRINT "e:"; LBOUND(e); UBOUND(e)
PRINT "UBOUND(b) / 7:"; UBOUND(b) / 7
PRINT "UBOUND(b) * 1000000000:"; UBOUND(b) * 1000000000
PRINT "LBOUND(b, 1.5); UBOUND(b, 2.5):"; LBOUND(b, 1.5); UBOUND(b, 2.5)
d = 2: PRINT "UBOUND(b, d):"; UBOUND(b, d)

' Every slice type: stores convert as for a variable of the element's type.
DIM i(1) AS INTEGER, l(1) AS LONG, q(1) AS _INTEGER64
DIM sg(1) AS SINGLE, db(1) AS DOUBLE, fl(1) AS _FLOAT
i(1) = 32767: l(1) = 2147483647: q(1) = 9223372036854775807
sg(1) = 1 / 3: db(1) = 1 / 3: fl(1) = 1 / 3
PRINT i(1); l(1); q(1)
PRINT sg(1); db(1); fl(1)
i(0) = 2.5: l(0) = 3.5: q(0) = -2.5
PRINT i(0); l(0); q(0)
i(1) = i(1) + 1
PRINT "i(1) + 1:"; i(1)
PRINT "l(1) / 3:"; l(1) / 3; " sg(1) * 3:"; sg(1) * 3

' Float indexes round half to even.
DIM x(10) AS LONG
FOR k = 0 TO 10: x(k) = k * 10: NEXT
PRINT "x(1.5) x(2.5) x(0.5) x(-0.5) x(10.5):"; x(1.5); x(2.5); x(0.5); x(-0.5); x(10.5)
sx! = 4.5: dx# = 5.5
PRINT "x(sx!) x(dx#):"; x(sx!); x(dx#)
x(7.5) = 1
PRINT "after x(7.5) = 1:"; x(7); x(8)
PRINT "x(x(1) / 10 + 2):"; x(x(1) / 10 + 2)

' Several dimensions.
DIM m(2, 3) AS LONG
FOR r = 0 TO 2: FOR c = 0 TO 3: m(r, c) = r * 10 + c: NEXT: NEXT
PRINT "m:"; m(0, 0); m(1, 2); m(2, 3); m(2, 0)
DIM cube(1, 1, 1) AS INTEGER
cube(1, 0, 1) = 5
PRINT "cube:"; cube(1, 0, 1); cube(0, 0, 0)

' Strings.
names(1) = "one": names(2) = names(1) + "+two"
t$(1) = "tee"
PRINT "names: ["; names(1); "] ["; names(2); "] ["; names(3); "]"
PRINT "t$(1): "; t$(1); " names$(1): "; names$(1)
PRINT "INSTR:"; INSTR(names(2), "two")

' An array and a scalar of the same name; a suffix naming the same array.
DIM a(3)
a = 5
a(1) = 2
PRINT "a; a(1):"; a; a(1)
DIM cl(3) AS LONG
cl(1) = 8
cl&(2) = 9
PRINT "cl(1); cl&(2); cl(2):"; cl(1); cl&(2); cl(2)

' A static array's DIM does nothing when it runs.
FOR k = 1 TO 2
    DIM again(5) AS LONG
    again(1) = again(1) + 1
    PRINT "pass"; k; again(1)
NEXT

' By reference and by copy.
x(3) = 10: bump x(3)
PRINT "bump x(3):"; x(3)
x(3) = 10: bump (x(3))
PRINT "bump (x(3)):"; x(3)
x(3) = 10: CALL bump(x(3))
PRINT "CALL bump(x(3)):"; x(3)
db(0) = 2.5: bump db(0)
PRINT "bump db(0) (a copy):"; db(0)
x(1) = 1: x(2) = 2
swapper x(1), x(2)
PRINT "swapper x(1), x(2):"; x(1); x(2)
x(3) = 11
PRINT "twice&(x(3)):"; twice&(x(3)); x(3)
names(3) = "s"
addbang names(3)
PRINT "addbang names(3): "; names(3)

' DIM SHARED arrays in a SUB and a FUNCTION.
s(1) = 1
fill
PRINT "s(2) after fill:"; s(2); total&
SYSTEM

SUB bump (v AS LONG)
    PRINT "in bump"; v
    v = v + 1
END SUB

SUB swapper (p AS LONG, q AS LONG)
    tmp& = p: p = q: q = tmp&
END SUB

FUNCTION twice& (v AS LONG)
    v = v * 2
    twice& = v
END FUNCTION

SUB addbang (t AS STRING)
    t = t + "!"
END SUB

SUB fill
    s(2) = s(1) + 1
END SUB

FUNCTION total&
    total& = s(1) + s(2)
END FUNCTION
