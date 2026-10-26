$Console:Only
Option _Explicit


Dim Shared PassCount As Long
Dim Shared FailCount As Long

Type PositiveBounds
    Grid(5 To 7, 10 To 11) As Integer
    Keep As Long
End Type

Dim a(0) As PositiveBounds
Dim b(0) As PositiveBounds
Dim i As Long
Dim j As Long
Dim m As _MEM

For i = 5 To 7
    For j = 10 To 11
        a(0).Grid(i, j) = i * 100 + j
    Next j
Next i
a(0).Keep = 1234
b(0).Keep = 5678

CheckTrue "bounds d1", LBound(a(0).Grid) = 5 And UBound(a(0).Grid) = 7
CheckTrue "bounds d2", LBound(a(0).Grid, 2) = 10 And UBound(a(0).Grid, 2) = 11
CheckTrue "values", a(0).Grid(5, 10) = 510 And a(0).Grid(7, 11) = 711

m = _Mem(a(0).Grid)
CheckTrue "mem size", m.SIZE = 12
CheckTrue "mem element size", m.ELEMENTSIZE = 2
_MemGet m, m.OFFSET, b(0).Grid
CheckTrue "memget copied", b(0).Grid(5, 10) = 510 And b(0).Grid(7, 11) = 711
CheckTrue "memget keeps scalar", b(0).Keep = 5678

Erase a(0).Grid
CheckTrue "erase member zeroes", a(0).Grid(5, 10) = 0 And a(0).Grid(7, 11) = 0
CheckTrue "erase member keeps scalar", a(0).Keep = 1234

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
