$Console:Only
Option _Explicit

Type ClearLeaf
    Texts(0 TO 255) As String
    Number As Long
End Type

Dim items(0 To 2) As ClearLeaf

items(0).Texts(0) = "zero"
items(1).Texts(128) = "middle"
items(2).Texts(255) = "last"
items(1).Number = 12345

' CLEAR must clear live qbs contents and numeric storage without leaving stale owners.
Clear

If items(0).Texts(0) <> "" Then GoTo failed
If items(1).Texts(128) <> "" Then GoTo failed
If items(2).Texts(255) <> "" Then GoTo failed
If items(1).Number <> 0 Then GoTo failed

' The array must remain usable after CLEAR.
items(2).Texts(255) = "after-clear"
items(2).Number = 99
If items(2).Texts(255) <> "after-clear" Then GoTo failed
If items(2).Number <> 99 Then GoTo failed

Print "PASS 95_type_codegen_clear_inline_owner.bas"
System

failed:
Print "FAIL 95_type_codegen_clear_inline_owner.bas"
System
