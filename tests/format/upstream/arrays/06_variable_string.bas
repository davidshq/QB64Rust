$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long


Type VariableStrings
    Names(0 To 2) As String
    Tags(0 To 1, 0 To 1) As String
    Title As String
    Count As Long
End Type

Dim a(0 To 1) As VariableStrings
Dim b As VariableStrings

a(0).Names(0) = "alpha"
a(0).Names(1) = "beta"
a(0).Names(2) = "gamma"
a(0).Tags(0, 0) = "red"
a(0).Tags(1, 1) = "blue"
a(0).Title = "source"
a(0).Count = 3

CheckText "Name 0", a(0).Names(0), "alpha"
CheckText "Name 2", a(0).Names(2), "gamma"
CheckText "Tag 1,1", a(0).Tags(1, 1), "blue"
CheckText "Title", a(0).Title, "source"
CheckTrue "Count", a(0).Count = 3

b = a(0)
CheckText "copy name", b.Names(1), "beta"
CheckText "copy tag", b.Tags(1, 1), "blue"
CheckText "copy title", b.Title, "source"

a(0).Names(1) = "changed"
CheckText "copy is independent for name", b.Names(1), "beta"

Erase a(0).Names
CheckText "ERASE member first", a(0).Names(0), ""
CheckText "ERASE member middle", a(0).Names(1), ""
CheckText "ERASE member last", a(0).Names(2), ""
CheckText "ERASE member keeps title", a(0).Title, "source"
CheckTrue "ERASE member keeps count", a(0).Count = 3

Erase a
CheckText "ERASE parent string member", a(0).Tags(1, 1), ""
CheckText "ERASE parent title", a(0).Title, ""
CheckTrue "ERASE parent count", a(0).Count = 0

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
