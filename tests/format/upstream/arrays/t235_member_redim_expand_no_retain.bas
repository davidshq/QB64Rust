$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T235
    nums(0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T235

work(0).nums(0) = 77
ReDim work(0).nums(0 TO 3)
ok = (LBound(work(0).nums) = 0 And UBound(work(0).nums) = 3 And work(0).nums(0) = 0 And work(0).nums(3) = 0)
If ok Then Print "PASS t235_member_redim_expand_no_retain" Else Print "FAIL t235_member_redim_expand_no_retain"
System
