$Console:Only
Option _Explicit


Dim Shared PassCount As Long
Dim Shared FailCount As Long


Type IntegerFamily
    AByte(0 To 3) As _Byte
    AUByte(0 To 3) As _Unsigned _Byte
    AInt(0 To 3) As Integer
    AUInt(0 To 3) As _Unsigned Integer
    ALong(0 To 3) As Long
    AULong(0 To 3) As _Unsigned Long
    AI64(0 To 3) As _Integer64
    AUI64(0 To 3) As _Unsigned _Integer64
End Type

Dim t As IntegerFamily
Dim i As Long
Dim j As Long
Dim k As Long

For i = 0 To 3
    t.AByte(i) = -10 - i
    t.AUByte(i) = 200 + i
    t.AInt(i) = -1000 - i
    t.AUInt(i) = 60000 + i
    t.ALong(i) = -100000 - i
    t.AULong(i) = 2000000000 + i
    t.AI64(i) = -9000000000 - i
    t.AUI64(i) = 900000000000 + i
Next i

CheckTrue "_Byte first", t.AByte(0) = -10
CheckTrue "_Byte last", t.AByte(3) = -13
CheckTrue "_Unsigned _Byte first", t.AUByte(0) = 200
CheckTrue "_Unsigned _Byte last", t.AUByte(3) = 203
CheckTrue "Integer first", t.AInt(0) = -1000
CheckTrue "Integer last", t.AInt(3) = -1003
CheckTrue "_Unsigned Integer first", t.AUInt(0) = 60000
CheckTrue "_Unsigned Integer last", t.AUInt(3) = 60003
CheckTrue "Long first", t.ALong(0) = -100000
CheckTrue "Long last", t.ALong(3) = -100003
CheckTrue "_Unsigned Long first", t.AULong(0) = 2000000000
CheckTrue "_Unsigned Long last", t.AULong(3) = 2000000003
CheckTrue "_Integer64 first", t.AI64(0) = -9000000000
CheckTrue "_Integer64 last", t.AI64(3) = -9000000003
CheckTrue "_Unsigned _Integer64 first", t.AUI64(0) = 900000000000
CheckTrue "_Unsigned _Integer64 last", t.AUI64(3) = 900000000003
CheckTrue "LBOUND 1D", LBound(t.AInt) = 0
CheckTrue "UBOUND 1D", UBound(t.AInt) = 3

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
