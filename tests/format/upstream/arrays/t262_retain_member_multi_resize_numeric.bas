$Console:Only
$Unstable:TypeFields

Option _Explicit

Type T262
    nums(0 TO 1) _Dynamic As Long
End Type

Dim ok As _Byte
ReDim work(0) As T262

work(0).nums(0) = 11
work(0).nums(1) = 22

ReDim _Retain work(0).nums(0 TO 4)
work(0).nums(2) = 33
work(0).nums(3) = 44
work(0).nums(4) = 55

ReDim _Retain work(0).nums(0 TO 2)
ok = (LBound(work(0).nums) = 0 And UBound(work(0).nums) = 2)
ok = ok And (work(0).nums(0) = 11 And work(0).nums(1) = 22 And work(0).nums(2) = 33)

ReDim _Retain work(0).nums(0 TO 6)
work(0).nums(5) = 66
work(0).nums(6) = 77

ok = ok And (LBound(work(0).nums) = 0 And UBound(work(0).nums) = 6)
ok = ok And (work(0).nums(0) = 11 And work(0).nums(1) = 22 And work(0).nums(2) = 33)
ok = ok And (work(0).nums(3) = 0 And work(0).nums(4) = 0 And work(0).nums(5) = 66 And work(0).nums(6) = 77)

If ok Then Print "PASS t262_retain_member_multi_resize_numeric" Else Print "FAIL t262_retain_member_multi_resize_numeric"
System
