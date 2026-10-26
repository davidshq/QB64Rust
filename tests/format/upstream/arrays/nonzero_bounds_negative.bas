$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long

Type NegativeBounds
    Grid(-3 To -1, -2 To 1) As Long
    Keep As Long
End Type

Dim x(0) As NegativeBounds
Dim i As Long
Dim j As Long
Dim total As Long
Dim m As _MEM

For i = -3 To -1
    For j = -2 To 1
        x(0).Grid(i, j) = i * 10 + j
        total = total + x(0).Grid(i, j)
    Next j
Next i
x(0).Keep = 999

CheckTrue "bounds d1", LBound(x(0).Grid) = -3 And UBound(x(0).Grid) = -1
CheckTrue "bounds d2", LBound(x(0).Grid, 2) = -2 And UBound(x(0).Grid, 2) = 1
CheckTrue "values", x(0).Grid(-3, -2) = -32 And x(0).Grid(-1, 1) = -9
CheckTrue "sum", total = -246

m = _Mem(x(0).Grid)
CheckTrue "mem size", m.SIZE = 48
CheckTrue "mem element size", m.ELEMENTSIZE = 4

Erase x(0).Grid
CheckTrue "erase zeroes", x(0).Grid(-3, -2) = 0 And x(0).Grid(-1, 1) = 0
CheckTrue "erase keeps scalar", x(0).Keep = 999

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
