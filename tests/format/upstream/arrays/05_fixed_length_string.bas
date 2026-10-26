$Console:Only
Option _Explicit

Dim Shared PassCount As Long
Dim Shared FailCount As Long

Type FixedStrings
    S4(0 TO 2) As String * 4
    S8(0 TO 1, 0 TO 1) As String * 8
    Label As String * 6
End Type

Dim a As FixedStrings
Dim b As FixedStrings
Dim z4 As String * 4

a.S4(0) = "AB"
a.S4(1) = "WXYZ"
a.S4(2) = "ABCDEFG"
a.S8(0, 0) = "HELLO"
a.S8(1, 1) = "123456789"
a.Label = "QB64PE"

CheckTrue "S4(0) len", Len(a.S4(0)) = 4
CheckText "S4(0) trimmed", RTrim$(a.S4(0)), "AB"
CheckText "S4(1) exact", a.S4(1), "WXYZ"
CheckText "S4(2) truncates", a.S4(2), "ABCD"

CheckTrue "S8(0,0) len", Len(a.S8(0, 0)) = 8
CheckText "S8(0,0) trimmed", RTrim$(a.S8(0, 0)), "HELLO"
CheckText "S8(1,1) truncates", a.S8(1, 1), "12345678"
CheckText "Label exact", a.Label, "QB64PE"

b = a
CheckText "copy S4(2)", b.S4(2), "ABCD"
CheckText "copy S8(1,1)", b.S8(1, 1), "12345678"
CheckText "copy Label", b.Label, "QB64PE"

Erase a.S4
CheckTrue "ERASE fixed member first", a.S4(0) = z4
CheckTrue "ERASE fixed member last", a.S4(2) = z4
CheckText "ERASE fixed member keeps label", a.Label, "QB64PE"

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

Sub FinishTest
    If FailCount = 0 Then
        Print "RESULT: PASS"
    Else
        Print "RESULT: FAIL"; FailCount; "failure(s)"
    End If
End Sub

