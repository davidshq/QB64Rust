' Graphics GET and PUT in SCREEN 12: each action word over a background, no action, a colour, _CLIP at two edges,
' STEP; the array whole and from an element, of two dimensions, of LONG, dynamic, and in a SUB as a parameter, a
' local and a STATIC array; a PUT that leaves the screen and a GET into an array that is too small raise error 5.
DefInt A-Z
Dim Shared errs
Dim a(150), m(1 To 4, 1 To 40), l(80) As Long, small(3)
Dim n
Screen 12
On Error GoTo handler
' The picture: a filled box with two diagonals and a dot.
Line (10, 10)-(30, 26), 4, BF
Line (10, 10)-(30, 26), 14
Line (10, 26)-(30, 10), 10
PSet (20, 18), 15
Get (10, 10)-(30, 26), a()
' A background for the actions.
Line (0, 60)-(639, 100), 3, BF
Put (50, 10), a()
Put (50, 70), a()
Put (100, 70), a(), PSet
Put (150, 70), a(), PReset
Put (200, 70), a(), And
Put (250, 70), a(), Or
Put (300, 70), a(), Xor
Put (350, 70), a(), , 4
Put (400, 70), a(), PSet , 14
Put (450, 70), a(), _Clip, 10
Put (630, 70), a(), _Clip PSet
Put (-10, 200), a(), _Clip
PSet (100, 150), 0
Put Step(10, 10), a(), PSet
' From an element; two dimensions; LONG; dynamic arrays.
Get (10, 10)-(20, 18), a(60)
Put (150, 150), a(60), PSet
Get (10, 10)-(30, 26), m()
Put (200, 150), m(), PSet
Get (10, 10)-(30, 26), m(1, 2)
Put (230, 150), m(1, 2), PSet
Get (10, 10)-(30, 26), l()
Put (260, 150), l(), PSet
n = 150
Dim dy(n)
Get (10, 10)-(30, 26), dy()
Put (300, 150), dy(), PSet
ReDim rd(1 To 150)
PSet (10, 10), 0
Get Step(0, 0)-Step(20, 16), rd(n - 149)
Put (350, 150), rd(1), Or
' In a SUB.
Call stamp(a(), 400, 150)
' Errors.
Put (630, 300), a(), PSet
Get (10, 10)-(30, 26), small()
Put (10, 300), small(), PSet
Get (10, 10)-(700, 26), a()
Locate 25, 1: Print "errors"; errs
System
handler:
errs = errs + 1
Locate 20 + errs, 1: Print "error"; Err
Resume Next

Sub stamp (p(), x, y)
    Dim lc(150)
    Static st(150)
    Put (x, y), p(), PSet
    Get (x, y)-(x + 20, y + 16), lc()
    Put (x + 50, y), lc(), PSet
    Get (10, 10)-(30, 26), st(2)
    Put (x + 100, y), st(2), Xor
    Put (x + 150, y), p(), PReset
End Sub
