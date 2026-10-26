$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long

Type InnerRecord
    Cells(2 To 4) As Integer
    Mark As Long
End Type

Type OuterRecord
    Items(0 To 1) As InnerRecord
    Footer As Long
End Type

Dim x(0) As OuterRecord

x(0).Items(0).Cells(2) = 12
x(0).Items(0).Cells(3) = 13
x(0).Items(0).Cells(4) = 14
x(0).Items(0).Mark = 100
x(0).Items(1).Cells(2) = 22
x(0).Items(1).Cells(3) = 23
x(0).Items(1).Cells(4) = 24
x(0).Items(1).Mark = 200
x(0).Footer = 300

CheckTrue "outer bounds", LBound(x(0).Items) = 0 And UBound(x(0).Items) = 1
CheckTrue "inner bounds", LBound(x(0).Items(1).Cells) = 2 And UBound(x(0).Items(1).Cells) = 4
CheckTrue "nested values 1", x(0).Items(0).Cells(2) = 12 And x(0).Items(1).Cells(4) = 24
CheckTrue "nested values 2", x(0).Items(0).Mark = 100 And x(0).Items(1).Mark = 200 And x(0).Footer = 300

Erase x(0).Items(1).Cells
CheckTrue "erase inner member array zeroes", x(0).Items(1).Cells(2) = 0 And x(0).Items(1).Cells(4) = 0
CheckTrue "erase inner member array keeps inner scalar", x(0).Items(1).Mark = 200
CheckTrue "erase inner member array keeps outer", x(0).Items(0).Cells(2) = 12 And x(0).Footer = 300

Erase x(0).Items
CheckTrue "erase outer member array clears nested 1", x(0).Items(0).Cells(2) = 0 And x(0).Items(1).Cells(4) = 0
CheckTrue "erase outer member array clears nested 2", x(0).Items(0).Mark = 0 And x(0).Items(1).Mark = 0
CheckTrue "erase outer member array keeps footer", x(0).Footer = 300

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
