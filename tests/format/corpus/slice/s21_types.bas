$Console:Only
' Slice program (m2-arrays-and-types, D9): TYPE with numeric and nested members. A type used above its block,
' variables in every storage class, member read and write, dotted names, members of array elements, members
' passed by reference. No PRINT comma.
Dim early As pt
early.x = 1

Type inner
    v As Long
    w As Integer
End Type

Type all
    i As Integer
    l As Long
    q As _Integer64
    s As Single
    d As Double
    f As _Float
    n As inner
End Type

Type pt
    x As Long
    y As Single
End Type

Type outer
    a As inner
    b As inner
    tag As Long
End Type

Dim p As all
Print "initial:"; p.i; p.l; p.q; p.s; p.d; p.f; p.n.v; p.n.w
p.i = 32767: p.l = 2147483647: p.q = 9223372036854775807
p.s = 1 / 3: p.d = 1 / 3: p.f = 1 / 3
p.n.v = 7: p.n.w = -2
Print p.i; p.l; p.q
Print p.s; p.d; p.f
Print p.n.v; p.n.w
p.i = 2.5: p.l = 3.5: p.q = -2.5
Print "rounded:"; p.i; p.l; p.q
p.i = p.i + 32767
Print "p.i wraps:"; p.i
Print "p.l / 3:"; p.l / 3; " p.s * 3:"; p.s * 3
Print "early.x:"; early.x

' Nesting, and members with their own suffix.
Dim o As outer
o.a.v = 1: o.b.v = 2: o.b.w = 3: o.tag = 4
Print "o:"; o.a.v; o.a.w; o.b.v; o.b.w; o.tag
o.a.v& = o.b.w% + 10
Print "o.a.v&:"; o.a.v&; o.a.v

' Dotted names: a plain variable without a TYPE variable of that name, and before the DIM.
plain.name = 3
plain = 4
Print "plain.name; plain:"; plain.name; plain
later.x = 30
Print "later.x before DIM:"; later.x
Dim later As pt
Print "later.x after DIM:"; later.x
later.x = 31
Print "later.x:"; later.x
Dim pt As pt
pt.x = 9
Print "pt.x:"; pt.x

' Arrays of a TYPE.
Dim arr(1 To 3) As pt
arr(1).x = 8: arr(2).y = 2.5: arr(3).x = arr(1).x * 2
Print "arr:"; arr(1).x; arr(1).y; arr(2).x; arr(2).y; arr(3).x
Print "LBOUND/UBOUND arr:"; LBound(arr); UBound(arr)
Dim grid(1, 1) As inner
grid(1, 0).v = 10: grid(0, 1).w = 11
Print "grid:"; grid(1, 0).v; grid(0, 1).w; grid(0, 0).v

' Members passed by reference; in parentheses a copy.
Dim Shared g As pt
g.x = 3
bump g.x
Print "bump g.x:"; g.x
bump (g.x)
Print "bump (g.x):"; g.x
bump arr(2).x
Print "bump arr(2).x:"; arr(2).x
bump o.b.v
Print "bump o.b.v:"; o.b.v
bump p.n.v
Print "bump p.n.v:"; p.n.v
bump p.i
Print "bump p.i (INTEGER member, a copy):"; p.i

' Storage classes.
Dim m As pt
m.x = 9
locsub
locsub
st
st
sh
Print "g.x after sh:"; g.x
Print "twice&(4):"; twice&(4)
System

Sub bump (v As Long)
    v = v + 1
End Sub

Sub locsub
    Dim lp As pt
    Print "local lp.x:"; lp.x
    lp.x = 5
End Sub

Sub st
    Static q As pt
    Print "static q.x:"; q.x
    q.x = q.x + 5
End Sub

Sub sh
    Shared m As pt
    Print "shared m.x:"; m.x; " g.x:"; g.x
    g.x = g.x + 1
End Sub

Function twice& (n As Long)
    Dim fp As pt
    fp.x = n * 2
    twice& = fp.x
End Function
