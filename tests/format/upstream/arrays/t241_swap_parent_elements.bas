$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T241
    nums(0 TO 1) _Dynamic As Long
    tagText As String
End Type

Dim ok As _Byte
ReDim work(0 To 1) As T241

work(0).nums(0) = 10
work(0).tagText = "zero"
work(1).nums(0) = 20
work(1).tagText = "one"
Swap work(0), work(1)
ok = (work(0).nums(0) = 20 And work(0).tagText = "one" And work(1).nums(0) = 10 And work(1).tagText = "zero")
If ok Then Print "PASS t241_swap_parent_elements" Else Print "FAIL t241_swap_parent_elements"
System
