$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T248
    nums(0 TO 1) _Dynamic As Long
    noteText As String
End Type

Dim ok As _Byte
ReDim work(0 To 0) As T248

work(0).nums(0) = 12
work(0).noteText = "keep"
ReDim _Retain work(0 To 1) As T248
work(1).nums(1) = 34
work(1).noteText = "new"
ok = (work(0).nums(0) = 12 And work(0).noteText = "keep" And work(1).nums(0) = 0 And work(1).nums(1) = 34 And work(1).noteText = "new")
If ok Then Print "PASS t248_parent_redim_retain_expands" Else Print "FAIL t248_parent_redim_retain_expands"
System
