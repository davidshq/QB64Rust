$Console:Only
$Unstable:TypeFields
' Slice (m3-typefields task 3.4): _DYNAMIC member arrays and procedures (verification\v35_d). A local of the type
' is new on each call, a STATIC local keeps its member, a SHARED variable, a parameter of the type and an element
' passed for it are by reference (a REDIM of the member in the callee is seen by the caller), the member passed as
' an array argument (its descriptor: the callee's REDIM changes it), a FUNCTION's parameter, the parent array as an
' argument, and the first bounds of a member named by a CONST.
Const top = 4
Type t
    n As Long
    v(1) _Dynamic As Long
End Type
Type c
    w(top) _Dynamic As Integer
End Type
On Error GoTo h
Dim Shared g As t
Dim x As t, k As c
Print "1. a local, twice:": loc: loc
Print "2. a STATIC local, twice:": sta: sta
Print "3. SHARED: before"; LBound(g.v); UBound(g.v);: sh: Print " after"; LBound(g.v); UBound(g.v); g.v(9)
Print "4. a parameter:";: par x: Print " caller sees"; LBound(x.v); UBound(x.v); x.v(3)
Print "5. the member as an array argument:";: arr x.v(): Print " caller sees"; LBound(x.v); UBound(x.v); x.v(7)
Print "6. a FUNCTION's parameter:"; total(x)
ReDim e(1) As t
ReDim e(1).v(2 TO 3) AS LONG: e(1).v(3) = 33
Print "7. an element as a parameter:";: par e(1): Print " caller sees"; LBound(e(1).v); UBound(e(1).v); e(1).v(3)
Print "8. the parent array as an argument:";: parr e(): Print " caller sees"; UBound(e(0).v); e(0).v(4)
Print "9. a bound named by a CONST:"; LBound(k.w); UBound(k.w)
System
h:
Print "[error"; Err; "]";
Resume Next

Sub loc
    Dim a As t
    Print "   "; LBound(a.v); UBound(a.v); a.v(1); a.n
    ReDim a.v(0 TO 4) AS LONG: a.v(1) = 5: a.n = 5
End Sub

Sub sta
    Static a As t
    Print "   "; LBound(a.v); UBound(a.v); a.v(1); a.n
    ReDim a.v(0 TO 4) AS LONG: a.v(1) = 5: a.n = 5
End Sub

Sub sh
    ReDim g.v(8 TO 9) AS LONG: g.v(9) = 99
End Sub

Sub par (p As t)
    Print " callee sees"; LBound(p.v); UBound(p.v);
    ReDim p.v(3 TO 3) AS LONG: p.v(3) = 3
End Sub

Sub arr (a() As Long)
    Print " callee sees"; LBound(a); UBound(a);
    ReDim a(6 To 7) As Long: a(7) = 7
End Sub

Sub parr (a() As t)
    Print " callee sees"; UBound(a); UBound(a(1).v);
    ReDim a(0).v(4) AS LONG: a(0).v(4) = 4
End Sub

Function total& (p As t)
    Dim i As Long, s As Long
    For i = LBound(p.v) To UBound(p.v): s = s + p.v(i): Next
    total& = s
End Function
