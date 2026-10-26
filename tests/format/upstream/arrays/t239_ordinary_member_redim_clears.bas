$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T239
    nums(0 TO 2) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T239

work(0).nums(0) = 7
work(0).nums(1) = 8
work(0).nums(2) = 9
ReDim work(0).nums(0 TO 2)
ok = (work(0).nums(0) = 0 And work(0).nums(1) = 0 And work(0).nums(2) = 0)
If ok Then Print "PASS t239_ordinary_member_redim_clears" Else Print "FAIL t239_ordinary_member_redim_clears"
System
