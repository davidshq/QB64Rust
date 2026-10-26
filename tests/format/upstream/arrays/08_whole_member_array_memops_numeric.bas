$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long

Type IntGridHolder
    G(0 To 2, 0 To 1) As Integer
End Type

Type DoubleGridHolder
    G(0 To 3) As Double
End Type

Dim a As IntGridHolder
Dim b As IntGridHolder
Dim da As DoubleGridHolder
Dim db As DoubleGridHolder
Dim mi As _MEM
Dim mdi As _MEM
Dim mdb As _MEM
Dim i As Long
Dim j As Long

For i = 0 To 2
    For j = 0 To 1
        a.G(i, j) = i * 10 + j + 1
        b.G(i, j) = 0
    Next j
Next i

CheckTrue "int LBOUND 1", LBound(a.G, 1) = 0
CheckTrue "int UBOUND 2", UBound(a.G, 2) = 1
CheckTrue "int _OFFSET base", _Offset(a.G) = _Offset(a.G(0, 0))

mi = _Mem(a.G)
CheckTrue "int _MEM size", mi.SIZE = 3 * 2 * 2
CheckTrue "int _MEM element size", mi.ELEMENTSIZE = 2

_MemGet mi, mi.OFFSET, b.G
CheckTrue "int _MEMGET first", b.G(0, 0) = 1
CheckTrue "int _MEMGET last", b.G(2, 1) = 22

b.G(0, 0) = 700
b.G(2, 1) = 701
_MemPut mi, mi.OFFSET, b.G
CheckTrue "int _MEMPUT first", a.G(0, 0) = 700
CheckTrue "int _MEMPUT last", a.G(2, 1) = 701

_MemFill mi, mi.OFFSET, mi.SIZE, 0 As Integer
CheckTrue "int _MEMFILL numeric first", a.G(0, 0) = 0
CheckTrue "int _MEMFILL numeric last", a.G(2, 1) = 0

For i = 0 To 2
    For j = 0 To 1
        b.G(i, j) = 50 + i * 10 + j
    Next j
Next i
_MemFill mi, mi.OFFSET, mi.SIZE, b.G
CheckTrue "int _MEMFILL array first", a.G(0, 0) = 50
CheckTrue "int _MEMFILL array last", a.G(2, 1) = 71

For i = 0 To 3
    da.G(i) = i * 2.5 + .25
    db.G(i) = 0
Next i

mdi = _Mem(da.G)
mdb = _Mem(db.G)
CheckTrue "double _MEM size", mdi.SIZE = 4 * 8
CheckTrue "double _MEM element size", mdi.ELEMENTSIZE = 8
_MemCopy mdi, mdi.OFFSET, mdi.SIZE To mdb, mdb.OFFSET
CheckNear "double _MEMCOPY first", db.G(0), .25, .0000001
CheckNear "double _MEMCOPY last", db.G(3), 7.75, .0000001

Erase da.G
CheckNear "double ERASE first", da.G(0), 0#, .0000001
CheckNear "double ERASE last", da.G(3), 0#, .0000001

_MemFree mi
_MemFree mdi
_MemFree mdb

FinishTest
System

Sub CheckTrue (label As String, condition As Long)
    If condition Then
        PassCount = PassCount + 1
    Else
        FailCount = FailCount + 1
        Print "FAIL:"; label
    End If
End Sub

Sub CheckText (label As String, actual As String, expected As String)
    If actual = expected Then
        PassCount = PassCount + 1
    Else
        FailCount = FailCount + 1
        Print "FAIL:"; label; " expected=["; expected; "] actual=["; actual; "]"
    End If
End Sub

Sub CheckNear (label As String, actual As Double, expected As Double, tolerance As Double)
    If Abs(actual - expected) <= tolerance Then
        PassCount = PassCount + 1
    Else
        FailCount = FailCount + 1
        Print "FAIL:"; label; " expected="; expected; " actual="; actual
    End If
End Sub

Sub FinishTest
    If FailCount = 0 Then
        Print "RESULT: PASS"
    Else
        Print "RESULT: FAIL"; FailCount; "failure(s)"
    End If
End Sub
