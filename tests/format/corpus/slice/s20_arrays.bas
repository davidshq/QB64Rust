$Console:Only
' Slice program (m2-arrays-and-types, D9): static arrays. Bounds, element read and write, every slice type,
' string arrays, LBOUND and UBOUND, elements passed by reference, DIM SHARED. No PRINT comma.
Const lo = -3, hi = 4
Dim b(-2 To 3, 5) As Long
Dim z(0) As Integer
Dim w(lo To hi) As Double
Dim f(2.5) As Long
Dim g(1.5 To 3.5) As Single
Dim e(-(2 ^ 2) To 10 \ 3) As _Integer64
Dim Shared s(5) As Long
Dim names(3) As String
Dim t$(2)

Print "initial:"; b(0, 0); z(0); w(lo); f(2); g(2); e(-4); "["; names(1); "]"; s(5)
b(-2, 0) = 1: b(3, 5) = 2: b(0, 3) = 3
Print "b:"; b(-2, 0); b(3, 5); b(0, 3); b(1, 3)
Print "LBOUND/UBOUND b:"; LBound(b); UBound(b); LBound(b, 1); UBound(b, 1); LBound(b, 2); UBound(b, 2)
Print "z:"; LBound(z); UBound(z)
Print "w:"; LBound(w); UBound(w)
Print "f:"; LBound(f); UBound(f)
Print "g:"; LBound(g); UBound(g)
Print "e:"; LBound(e); UBound(e)
Print "UBOUND(b) / 7:"; UBound(b) / 7
Print "UBOUND(b) * 1000000000:"; UBound(b) * 1000000000
Print "LBOUND(b, 1.5); UBOUND(b, 2.5):"; LBound(b, 1.5); UBound(b, 2.5)
d = 2: Print "UBOUND(b, d):"; UBound(b, d)

' Every slice type: stores convert as for a variable of the element's type.
Dim i(1) As Integer, l(1) As Long, q(1) As _Integer64
Dim sg(1) As Single, db(1) As Double, fl(1) As _Float
i(1) = 32767: l(1) = 2147483647: q(1) = 9223372036854775807
sg(1) = 1 / 3: db(1) = 1 / 3: fl(1) = 1 / 3
Print i(1); l(1); q(1)
Print sg(1); db(1); fl(1)
i(0) = 2.5: l(0) = 3.5: q(0) = -2.5
Print i(0); l(0); q(0)
i(1) = i(1) + 1
Print "i(1) + 1:"; i(1)
Print "l(1) / 3:"; l(1) / 3; " sg(1) * 3:"; sg(1) * 3

' Float indexes round half to even.
Dim x(10) As Long
For k = 0 To 10: x(k) = k * 10: Next
Print "x(1.5) x(2.5) x(0.5) x(-0.5) x(10.5):"; x(1.5); x(2.5); x(0.5); x(-0.5); x(10.5)
sx! = 4.5: dx# = 5.5
Print "x(sx!) x(dx#):"; x(sx!); x(dx#)
x(7.5) = 1
Print "after x(7.5) = 1:"; x(7); x(8)
Print "x(x(1) / 10 + 2):"; x(x(1) / 10 + 2)

' Several dimensions.
Dim m(2, 3) As Long
For r = 0 To 2: For c = 0 To 3: m(r, c) = r * 10 + c: Next: Next
Print "m:"; m(0, 0); m(1, 2); m(2, 3); m(2, 0)
Dim cube(1, 1, 1) As Integer
cube(1, 0, 1) = 5
Print "cube:"; cube(1, 0, 1); cube(0, 0, 0)

' Strings.
names(1) = "one": names(2) = names(1) + "+two"
t$(1) = "tee"
Print "names: ["; names(1); "] ["; names(2); "] ["; names(3); "]"
Print "t$(1): "; t$(1); " names$(1): "; names$(1)
Print "INSTR:"; InStr(names(2), "two")

' An array and a scalar of the same name; a suffix naming the same array.
Dim a(3)
a = 5
a(1) = 2
Print "a; a(1):"; a; a(1)
Dim cl(3) As Long
cl(1) = 8
cl&(2) = 9
Print "cl(1); cl&(2); cl(2):"; cl(1); cl&(2); cl(2)

' A static array's DIM does nothing when it runs.
For k = 1 To 2
    Dim again(5) As Long
    again(1) = again(1) + 1
    Print "pass"; k; again(1)
Next

' By reference and by copy.
x(3) = 10: bump x(3)
Print "bump x(3):"; x(3)
x(3) = 10: bump (x(3))
Print "bump (x(3)):"; x(3)
x(3) = 10: Call bump(x(3))
Print "CALL bump(x(3)):"; x(3)
db(0) = 2.5: bump db(0)
Print "bump db(0) (a copy):"; db(0)
x(1) = 1: x(2) = 2
swapper x(1), x(2)
Print "swapper x(1), x(2):"; x(1); x(2)
x(3) = 11
Print "twice&(x(3)):"; twice&(x(3)); x(3)
names(3) = "s"
addbang names(3)
Print "addbang names(3): "; names(3)

' DIM SHARED arrays in a SUB and a FUNCTION.
s(1) = 1
fill
Print "s(2) after fill:"; s(2); total&
System

Sub bump (v As Long)
    Print "in bump"; v
    v = v + 1
End Sub

Sub swapper (p As Long, q As Long)
    tmp& = p: p = q: q = tmp&
End Sub

Function twice& (v As Long)
    v = v * 2
    twice& = v
End Function

Sub addbang (t As String)
    t = t + "!"
End Sub

Sub fill
    s(2) = s(1) + 1
End Sub

Function total&
    total& = s(1) + s(2)
End Function
