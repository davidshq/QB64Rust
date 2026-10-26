$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T247
    nums(0 TO 1) _Dynamic As Long
    noteText As String
End Type

Dim ok As _Byte
ReDim work(0) As T247

work(0).nums(0) = 77
work(0).noteText = "old"
ReDim work(0) As T247
ok = (work(0).nums(0) = 0 And work(0).nums(1) = 0 And work(0).noteText = "")
If ok Then Print "PASS t247_parent_redim_ordinary_resets" Else Print "FAIL t247_parent_redim_ordinary_resets"
System
