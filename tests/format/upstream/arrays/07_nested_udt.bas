$Console:Only
Option _Explicit


Dim Shared PassCount As Long
Dim Shared FailCount As Long


Type Cell
    Value As Long
    Note As String
End Type

Type Board
    Grid(0 To 1, 0 To 2) As Integer
    Cells(0 To 1) As Cell
    Name As String * 6
End Type

Type World
    Boards(0 To 1) As Board
    Labels(0 To 1) As String
End Type

Dim a(0) As World
Dim b(0) As World

a(0).Boards(1).Grid(1, 2) = 123
a(0).Boards(0).Cells(1).Value = 456
a(0).Boards(0).Cells(1).Note = "ok"
a(0).Boards(1).Name = "BOARD2"
a(0).Labels(0) = "earth"
a(0).Labels(1) = "mars"

CheckTrue "nested grid value", a(0).Boards(1).Grid(1, 2) = 123
CheckTrue "nested cell value", a(0).Boards(0).Cells(1).Value = 456
CheckText "nested cell note", a(0).Boards(0).Cells(1).Note, "ok"
CheckText "board name", a(0).Boards(1).Name, "BOARD2"
CheckText "label 1", a(0).Labels(1), "mars"

b() = a()
CheckTrue "copy nested grid", b(0).Boards(1).Grid(1, 2) = 123
CheckText "copy nested note", b(0).Boards(0).Cells(1).Note, "ok"
CheckText "copy label", b(0).Labels(0), "earth"

Erase a(0).Boards(1).Grid
CheckTrue "ERASE nested grid resets value", a(0).Boards(1).Grid(1, 2) = 0
CheckText "ERASE nested grid keeps board name", a(0).Boards(1).Name, "BOARD2"
CheckText "ERASE nested grid keeps world label", a(0).Labels(1), "mars"

Erase a
CheckText "ERASE parent nested string", a(0).Boards(0).Cells(1).Note, ""
CheckTrue "ERASE parent nested numeric", a(0).Boards(0).Cells(1).Value = 0
CheckText "ERASE parent outer label", a(0).Labels(0), ""

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
