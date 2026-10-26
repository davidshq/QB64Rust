$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T237
    nums(0 TO 3) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T237

work(0).nums(0) = 10
work(0).nums(1) = 20
work(0).nums(2) = 30
work(0).nums(3) = 40
ReDim _Retain work(0).nums(0 TO 1)
ok = (LBound(work(0).nums) = 0 And UBound(work(0).nums) = 1 And work(0).nums(0) = 10 And work(0).nums(1) = 20)
If ok Then Print "PASS t237_member_redim_retain_shrink" Else Print "FAIL t237_member_redim_retain_shrink"
System
