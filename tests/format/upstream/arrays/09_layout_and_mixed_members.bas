$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long


Type MixedLayout
    Prefix As Long
    A(0 To 2) As Integer
    Middle As Double
    B(0 To 1, 0 To 1) As Long
    Tail As _Unsigned _Byte
End Type

Dim x(0 To 0) As MixedLayout
Dim memA As _MEM
Dim memB As _MEM

x(0).Prefix = 111
x(0).A(0) = 1
x(0).A(1) = 2
x(0).A(2) = 3
x(0).Middle = 4.5
x(0).B(0, 0) = 10
x(0).B(1, 1) = 20
x(0).Tail = 255

CheckTrue "offset A after Prefix", _Offset(x(0).A) > _Offset(x(0).Prefix)
CheckTrue "offset Middle after A", _Offset(x(0).Middle) > _Offset(x(0).A)
CheckTrue "offset B after Middle", _Offset(x(0).B) > _Offset(x(0).Middle)
CheckTrue "offset Tail after B", _Offset(x(0).Tail) > _Offset(x(0).B)

memA = _Mem(x(0).A)
memB = _Mem(x(0).B)
CheckTrue "memA element size", memA.ELEMENTSIZE = 2
CheckTrue "memB element size", memB.ELEMENTSIZE = 4

Erase x(0).A
CheckTrue "ERASE A zeroes member", x(0).A(0) = 0 And x(0).A(2) = 0
CheckTrue "ERASE A keeps Prefix", x(0).Prefix = 111
CheckNear "ERASE A keeps Middle", x(0).Middle, 4.5, .0000001
CheckTrue "ERASE A keeps B", x(0).B(1, 1) = 20
CheckTrue "ERASE A keeps Tail", x(0).Tail = 255

Erase x
CheckTrue "ERASE parent zeroes Prefix", x(0).Prefix = 0
CheckNear "ERASE parent zeroes Middle", x(0).Middle, 0#, .0000001
CheckTrue "ERASE parent zeroes B", x(0).B(1, 1) = 0
CheckTrue "ERASE parent zeroes Tail", x(0).Tail = 0

_MemFree memA
_MemFree memB

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
