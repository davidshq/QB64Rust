$Console:Only
Option _Explicit


Dim Shared PassCount As Long
Dim Shared FailCount As Long

Type MemOpsRecord
    Header As Long
    Grid(2 To 4) As Integer
    Footer As Long
End Type

Dim a(0) As MemOpsRecord
Dim b(0) As MemOpsRecord
Dim m As _MEM

Dim i As Long

a(0).Header = 111
For i = 2 To 4
    a(0).Grid(i) = i * 11
Next i
a(0).Footer = 222

b(0).Header = 333
For i = 2 To 4
    b(0).Grid(i) = 0
Next i
b(0).Footer = 444

CheckTrue "bounds", LBound(a(0).Grid) = 2 And UBound(a(0).Grid) = 4
m = _Mem(a(0).Grid)
CheckTrue "mem size", m.SIZE = 6
CheckTrue "mem element size", m.ELEMENTSIZE = 2

_MemGet m, m.OFFSET, b(0).Grid
CheckTrue "memget copied", b(0).Grid(2) = 22 And b(0).Grid(4) = 44
CheckTrue "memget keeps scalars", b(0).Header = 333 And b(0).Footer = 444

b(0).Grid(2) = 7
b(0).Grid(3) = 8
b(0).Grid(4) = 9
_MemPut m, m.OFFSET, b(0).Grid
CheckTrue "memput copied back", a(0).Grid(2) = 7 And a(0).Grid(4) = 9
CheckTrue "memput keeps scalars", a(0).Header = 111 And a(0).Footer = 222

_MemFill m, m.OFFSET, m.SIZE, 0 As Integer
CheckTrue "memfill zeroes", a(0).Grid(2) = 0 And a(0).Grid(4) = 0
CheckTrue "memfill keeps scalars", a(0).Header = 111 And a(0).Footer = 222

_MemFree m
FinishTest
'SLEEP
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
