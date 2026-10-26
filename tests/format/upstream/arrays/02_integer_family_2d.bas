$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long


Type IntegerFamily
    AByte(0 To 1, 0 To 2) As _Byte
    AUByte(0 To 1, 0 To 2) As _Unsigned _Byte
    AInt(0 To 1, 0 To 2) As Integer
    AUInt(0 To 1, 0 To 2) As _Unsigned Integer
    ALong(0 To 1, 0 To 2) As Long
    AULong(0 To 1, 0 To 2) As _Unsigned Long
    AI64(0 To 1, 0 To 2) As _Integer64
    AUI64(0 To 1, 0 To 2) As _Unsigned _Integer64
End Type

Dim t As IntegerFamily
Dim i As Long
Dim j As Long
Dim k As Long

For i = 0 To 1
    For j = 0 To 2
        t.AByte(i, j) = -10 - i - j
        t.AUByte(i, j) = 200 + i + j
        t.AInt(i, j) = -1000 - i * 10 - j
        t.AUInt(i, j) = 60000 + i * 10 + j
        t.ALong(i, j) = -100000 - i * 10 - j
        t.AULong(i, j) = 2000000000 + i * 10 + j
        t.AI64(i, j) = -9000000000 - i * 10 - j
        t.AUI64(i, j) = 900000000000 + i * 10 + j
    Next j
Next i

CheckTrue "_Byte sample", t.AByte(1, 2) = -13
CheckTrue "_Unsigned _Byte sample", t.AUByte(1, 2) = 203
CheckTrue "Integer sample", t.AInt(1, 2) = -1012
CheckTrue "_Unsigned Integer sample", t.AUInt(1, 2) = 60012
CheckTrue "Long sample", t.ALong(1, 2) = -100012
CheckTrue "_Unsigned Long sample", t.AULong(1, 2) = 2000000012
CheckTrue "_Integer64 sample", t.AI64(1, 2) = -9000000012
CheckTrue "_Unsigned _Integer64 sample", t.AUI64(1, 2) = 900000000012
CheckTrue "LBOUND dim1", LBound(t.AInt, 1) = 0
CheckTrue "UBOUND dim2", UBound(t.AInt, 2) = 2

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
