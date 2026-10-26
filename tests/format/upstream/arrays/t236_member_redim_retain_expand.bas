$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T236
    nums(0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T236

work(0).nums(0) = 11
work(0).nums(1) = 22
ReDim _Retain work(0).nums(0 TO 3)
work(0).nums(2) = 33
work(0).nums(3) = 44
ok = (LBound(work(0).nums) = 0 And UBound(work(0).nums) = 3 And work(0).nums(0) = 11 And work(0).nums(1) = 22 And work(0).nums(2) = 33 And work(0).nums(3) = 44)
If ok Then Print "PASS t236_member_redim_retain_expand" Else Print "FAIL t236_member_redim_retain_expand"
System
