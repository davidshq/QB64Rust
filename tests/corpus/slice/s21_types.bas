$CONSOLE:ONLY
' Slice program (m2-arrays-and-types, D9): TYPE with numeric and nested members. A type used above its block,
' variables in every storage class, member read and write, dotted names, members of array elements, members
' passed by reference. No PRINT comma.
DIM early AS pt
early.x = 1

TYPE inner
    v AS LONG
    w AS INTEGER
END TYPE

TYPE all
    i AS INTEGER
    l AS LONG
    q AS _INTEGER64
    s AS SINGLE
    d AS DOUBLE
    f AS _FLOAT
    n AS inner
END TYPE

TYPE pt
    x AS LONG
    y AS SINGLE
END TYPE

TYPE outer
    a AS inner
    b AS inner
    tag AS LONG
END TYPE

DIM p AS all
PRINT "initial:"; p.i; p.l; p.q; p.s; p.d; p.f; p.n.v; p.n.w
p.i = 32767: p.l = 2147483647: p.q = 9223372036854775807
p.s = 1 / 3: p.d = 1 / 3: p.f = 1 / 3
p.n.v = 7: p.n.w = -2
PRINT p.i; p.l; p.q
PRINT p.s; p.d; p.f
PRINT p.n.v; p.n.w
p.i = 2.5: p.l = 3.5: p.q = -2.5
PRINT "rounded:"; p.i; p.l; p.q
p.i = p.i + 32767
PRINT "p.i wraps:"; p.i
PRINT "p.l / 3:"; p.l / 3; " p.s * 3:"; p.s * 3
PRINT "early.x:"; early.x

' Nesting, and members with their own suffix.
DIM o AS outer
o.a.v = 1: o.b.v = 2: o.b.w = 3: o.tag = 4
PRINT "o:"; o.a.v; o.a.w; o.b.v; o.b.w; o.tag
o.a.v& = o.b.w% + 10
PRINT "o.a.v&:"; o.a.v&; o.a.v

' Dotted names: a plain variable without a TYPE variable of that name, and before the DIM.
plain.name = 3
plain = 4
PRINT "plain.name; plain:"; plain.name; plain
later.x = 30
PRINT "later.x before DIM:"; later.x
DIM later AS pt
PRINT "later.x after DIM:"; later.x
later.x = 31
PRINT "later.x:"; later.x
DIM pt AS pt
pt.x = 9
PRINT "pt.x:"; pt.x

' Arrays of a TYPE.
DIM arr(1 TO 3) AS pt
arr(1).x = 8: arr(2).y = 2.5: arr(3).x = arr(1).x * 2
PRINT "arr:"; arr(1).x; arr(1).y; arr(2).x; arr(2).y; arr(3).x
PRINT "LBOUND/UBOUND arr:"; LBOUND(arr); UBOUND(arr)
DIM grid(1, 1) AS inner
grid(1, 0).v = 10: grid(0, 1).w = 11
PRINT "grid:"; grid(1, 0).v; grid(0, 1).w; grid(0, 0).v

' Members passed by reference; in parentheses a copy.
DIM SHARED g AS pt
g.x = 3
bump g.x
PRINT "bump g.x:"; g.x
bump (g.x)
PRINT "bump (g.x):"; g.x
bump arr(2).x
PRINT "bump arr(2).x:"; arr(2).x
bump o.b.v
PRINT "bump o.b.v:"; o.b.v
bump p.n.v
PRINT "bump p.n.v:"; p.n.v
bump p.i
PRINT "bump p.i (INTEGER member, a copy):"; p.i

' Storage classes.
DIM m AS pt
m.x = 9
locsub
locsub
st
st
sh
PRINT "g.x after sh:"; g.x
PRINT "twice&(4):"; twice&(4)
SYSTEM

SUB bump (v AS LONG)
    v = v + 1
END SUB

SUB locsub
    DIM lp AS pt
    PRINT "local lp.x:"; lp.x
    lp.x = 5
END SUB

SUB st
    STATIC q AS pt
    PRINT "static q.x:"; q.x
    q.x = q.x + 5
END SUB

SUB sh
    SHARED m AS pt
    PRINT "shared m.x:"; m.x; " g.x:"; g.x
    g.x = g.x + 1
END SUB

FUNCTION twice& (n AS LONG)
    DIM fp AS pt
    fp.x = n * 2
    twice& = fp.x
END FUNCTION
