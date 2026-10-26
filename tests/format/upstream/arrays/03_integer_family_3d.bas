$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long


Type IntegerFamily
    AByte(0 To 1, 0 To 1, 0 To 1) As _Byte
    AUByte(0 To 1, 0 To 1, 0 To 1) As _Unsigned _Byte
    AInt(0 To 1, 0 To 1, 0 To 1) As Integer
    AUInt(0 To 1, 0 To 1, 0 To 1) As _Unsigned Integer
    ALong(0 To 1, 0 To 1, 0 To 1) As Long
    AULong(0 To 1, 0 To 1, 0 To 1) As _Unsigned Long
    AI64(0 To 1, 0 To 1, 0 To 1) As _Integer64
    AUI64(0 To 1, 0 To 1, 0 To 1) As _Unsigned _Integer64
End Type

Dim t As IntegerFamily
Dim i As Long
Dim j As Long
Dim k As Long

For i = 0 To 1
    For j = 0 To 1
        For k = 0 To 1
            t.AByte(i, j, k) = -10 - i - j - k
            t.AUByte(i, j, k) = 200 + i + j + k
            t.AInt(i, j, k) = -1000 - i * 100 - j * 10 - k
            t.AUInt(i, j, k) = 60000 + i * 100 + j * 10 + k
            t.ALong(i, j, k) = -100000 - i * 100 - j * 10 - k
            t.AULong(i, j, k) = 2000000000 + i * 100 + j * 10 + k
            t.AI64(i, j, k) = -9000000000 - i * 100 - j * 10 - k
            t.AUI64(i, j, k) = 900000000000 + i * 100 + j * 10 + k
        Next k
    Next j
Next i

CheckTrue "_Byte sample", t.AByte(1, 1, 1) = -13
CheckTrue "_Unsigned _Byte sample", t.AUByte(1, 1, 1) = 203
CheckTrue "Integer sample", t.AInt(1, 1, 1) = -1111
CheckTrue "_Unsigned Integer sample", t.AUInt(1, 1, 1) = 60111
CheckTrue "Long sample", t.ALong(1, 1, 1) = -100111
CheckTrue "_Unsigned Long sample", t.AULong(1, 1, 1) = 2000000111
CheckTrue "_Integer64 sample", t.AI64(1, 1, 1) = -9000000111
CheckTrue "_Unsigned _Integer64 sample", t.AUI64(1, 1, 1) = 900000000111
CheckTrue "LBOUND dim3", LBound(t.AInt, 3) = 0
CheckTrue "UBOUND dim2", UBound(t.AInt, 2) = 1

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
