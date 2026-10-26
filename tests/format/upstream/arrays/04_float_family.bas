$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long


Type FloatFamily
    S(0 To 2, 0 To 1) As Single
    D(0 To 2, 0 To 1) As Double
    F(0 To 2, 0 To 1) As _Float
End Type

Dim a As FloatFamily
Dim b As FloatFamily
Dim i As Long
Dim j As Long


For i = 0 To 2
    For j = 0 To 1
        a.S(i, j) = i * 1.25 + j * .5
        a.D(i, j) = i * 10.125 + j * .25
        a.F(i, j) = i * 100.5 + j * .125
    Next j
Next i

CheckNear "Single sample", a.S(2, 1), 3#, .0001
CheckNear "Double sample", a.D(2, 1), 20.5, .0000001
CheckNear "_Float sample", a.F(2, 1), 201.125, .0000001
CheckTrue "LBOUND dim1", LBound(a.S, 1) = 0
CheckTrue "UBOUND dim2", UBound(a.S, 2) = 1

b = a
CheckNear "copy Single", b.S(2, 1), 3#, .0001
CheckNear "copy Double", b.D(2, 1), 20.5, .0000001
CheckNear "copy _Float", b.F(2, 1), 201.125, .0000001

Erase a.S
CheckNear "ERASE Single first", a.S(0, 0), 0#, .0001
CheckNear "ERASE Single last", a.S(2, 1), 0#, .0001
CheckNear "ERASE keeps Double intact", a.D(2, 1), 20.5, .0000001
CheckNear "ERASE keeps _Float intact", a.F(2, 1), 201.125, .0000001

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
